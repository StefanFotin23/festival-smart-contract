#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait FestivalSmartContract {
    #[init]
    fn init(&self) {
        self.festival_count().set(0);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // ========================================================================
    // STORAGE
    // ========================================================================

    #[storage_mapper("festivalCount")]
    fn festival_count(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("ticketTokenIdentifier")]
    fn ticket_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    // Festival Data
    #[storage_mapper("festivalName")]
    fn festival_name(&self, id: u64) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("festivalConfig")]
    fn festival_config(&self, id: u64) -> SingleValueMapper<(u64, u64, u64)>;

    #[storage_mapper("festivalState")]
    fn festival_state(&self, id: u64) -> SingleValueMapper<(u64, u64)>;

    #[storage_mapper("festivalTax")]
    fn festival_tax(&self, id: u64) -> SingleValueMapper<(u8, u8)>;

    // Lists
    #[storage_mapper("events")]
    fn events(&self, festival_id: u64) -> VecMapper<(ManagedBuffer, ManagedBuffer, u64, u64)>;

    #[storage_mapper("announcements")]
    fn announcements(&self, festival_id: u64) -> VecMapper<(ManagedBuffer, u64)>;

    #[storage_mapper("flashEvents")]
    fn flash_events(&self, festival_id: u64) -> VecMapper<(ManagedBuffer, u64, u64, u64)>;

    #[storage_mapper("ticketPrices")]
    fn ticket_prices(&self, festival_id: u64) -> VecMapper<(ManagedBuffer, ManagedBuffer, BigUint)>;

    // Participants
    #[storage_mapper("userList")]
    fn user_list(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("userName")]
    fn user_name(&self, address: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("userScore")]
    fn user_score(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("userTimeData")]
    fn user_time_data(&self, address: &ManagedAddress) -> SingleValueMapper<(u64, u64)>;

    #[storage_mapper("claimedFlashEvents")]
    fn claimed_flash_events(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("resaleInfo")]
    fn resale_info(&self, ticket_nonce: u64) -> SingleValueMapper<(ManagedAddress, u64, BigUint)>;

    // ========================================================================
    // ENDPOINTS
    // ========================================================================

    #[only_owner]
    #[endpoint(addFestival)]
    fn add_festival(
        &self,
        name: ManagedBuffer,
        start_time: u64,
        end_time: u64,
        max_tickets: u64,
        tax_normal: u8,
        tax_sold_out: u8,
    ) {
        let new_id = self.festival_count().get() + 1;
        self.festival_count().set(new_id);

        self.festival_name(new_id).set(name);
        self.festival_config(new_id).set((start_time, end_time, max_tickets));
        self.festival_state(new_id).set((0, 0));
        self.festival_tax(new_id).set((tax_normal, tax_sold_out));
    }

    #[only_owner]
    #[endpoint(addEvent)]
    fn add_event(
        &self,
        festival_id: u64,
        name: ManagedBuffer,
        location: ManagedBuffer,
        start_time: u64,
        end_time: u64,
    ) {
        self.events(festival_id).push(&(name, location, start_time, end_time));
    }

    #[only_owner]
    #[endpoint(addTicketPrice)]
    fn add_ticket_price(
        &self,
        festival_id: u64,
        name: ManagedBuffer,
        phase: ManagedBuffer,
        price: BigUint,
    ) {
        self.ticket_prices(festival_id).push(&(name, phase, price));
    }

    #[only_owner]
    #[endpoint(addFlashEvent)]
    fn add_flash_event(
        &self,
        festival_id: u64,
        name: ManagedBuffer,
        start_time: u64,
        end_time: u64,
        bonus_points: u64,
    ) {
        self.flash_events(festival_id).push(&(name, start_time, end_time, bonus_points));
    }

    #[only_owner]
    #[endpoint(setTicketTokenIdentifier)]
    fn set_ticket_token_identifier(&self, token_identifier: TokenIdentifier) {
        self.ticket_token_identifier().set(&token_identifier);
    }

    // FIXED: Correct Minting Logic (Create -> Send)
    #[payable("EGLD")]
    #[endpoint(buyTicket)]
    fn buy_ticket(&self, festival_id: u64, ticket_price_name: ManagedBuffer) {
        let payment = self.call_value().egld().clone_value();
        let caller = self.blockchain().get_caller();

        let (_start, _end, max) = self.festival_config(festival_id).get();
        let (sold, inside) = self.festival_state(festival_id).get();
        require!(sold < max, "Tickets are sold out");

        // Check Price
        let mut found_price = BigUint::zero();
        let mut found = false;
        for item in self.ticket_prices(festival_id).iter() {
            let (p_name, _p_phase, p_price) = item; 
            if p_name == ticket_price_name {
                found_price = p_price;
                found = true;
                break;
            }
        }
        require!(found, "Price category not found");
        require!(payment == found_price, "Payment amount is incorrect");

        // 1. Create NFT (Mints to Contract)
        let token_identifier = self.ticket_token_identifier().get();
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(b"https://myfestival.com/ticket.json")); 

        let new_nonce = self.send().esdt_nft_create(
            &token_identifier,
            &BigUint::from(1u64), // Amount
            &ManagedBuffer::from("Festival Ticket"), // Name
            &BigUint::zero(), // Royalties
            &ManagedBuffer::new(), // Hash
            &ManagedBuffer::new(), // Attributes
            &uris,
        );

        // 2. Send NFT to Buyer
        self.send().direct_esdt(&caller, &token_identifier, new_nonce, &BigUint::from(1u64));

        self.festival_state(festival_id).set((sold + 1, inside));
        self.ticket_bought_event(&caller, festival_id, new_nonce);
    }

    #[endpoint(createParticipant)]
    fn create_participant(&self, username: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        require!(!self.user_list().contains(&caller), "Participant already exists");

        self.user_list().insert(caller.clone());
        self.user_name(&caller).set(username.clone());
        self.user_score(&caller).set(0);
        self.user_time_data(&caller).set((0, 0)); 

        self.participant_created_event(&caller, &username);
    }

    #[endpoint(checkIn)]
    fn check_in(&self, festival_id: u64) {
        let caller = self.blockchain().get_caller();
        require!(self.user_list().contains(&caller), "You are not a participant");

        let (_last_check_in, total_time) = self.user_time_data(&caller).get();
        self.user_time_data(&caller).set((self.blockchain().get_block_timestamp(), total_time));

        let (sold, inside) = self.festival_state(festival_id).get();
        self.festival_state(festival_id).set((sold, inside + 1));
    }

    #[endpoint(checkOut)]
    fn check_out(&self, festival_id: u64) {
        let caller = self.blockchain().get_caller();
        require!(self.user_list().contains(&caller), "You are not a participant");

        let (last_check_in, total_time) = self.user_time_data(&caller).get();
        require!(last_check_in > 0, "Not checked in");

        let time_now = self.blockchain().get_block_timestamp();
        let session_time = time_now - last_check_in;
        let new_total_time = total_time + session_time;

        let hours_spent = new_total_time / 3600;
        let current_score = self.user_score(&caller).get();
        
        if hours_spent > current_score {
            self.user_score(&caller).set(hours_spent);
        }

        self.user_time_data(&caller).set((0, new_total_time));

        let (sold, inside) = self.festival_state(festival_id).get();
        if inside > 0 {
            self.festival_state(festival_id).set((sold, inside - 1));
        }
    }

    #[endpoint(claimFlashEventPoints)]
    fn claim_flash_event_points(&self, festival_id: u64, flash_event_index: usize) {
        let caller = self.blockchain().get_caller();
        require!(self.user_list().contains(&caller), "You are not a participant");

        let flash_event = self.flash_events(festival_id).get(flash_event_index);
        let (_name, start, end, bonus) = flash_event;

        let current_time = self.blockchain().get_block_timestamp();
        require!(current_time >= start && current_time <= end, "Flash event is not active");
        
        let unique_id = festival_id * 1_000_000 + (flash_event_index as u64);

        require!(!self.claimed_flash_events(&caller).contains(&unique_id), "Already claimed");

        self.user_score(&caller).update(|score| *score += bonus);
        self.claimed_flash_events(&caller).insert(unique_id);
    }

    // FIXED: Added .clone() to fix the move error
    #[payable("EGLD")]
    #[endpoint(putTicketForSale)]
    fn put_ticket_for_sale(&self, festival_id: u64, price: BigUint) {
        let (payment_token, payment_nonce, payment_amount) = self.call_value().single_esdt().clone().into_tuple();
        
        require!(payment_token == self.ticket_token_identifier().get(), "Wrong token sent");
        require!(payment_amount == 1, "Must send exactly 1 ticket");

        let caller = self.blockchain().get_caller();
        
        // We now hold the ticket. We save the seller info so we know who to pay later.
        self.resale_info(payment_nonce).set((caller, festival_id, price));
    }

    // FIXED: Now simply transfers the ticket from Contract -> Buyer
    #[payable("EGLD")]
    #[endpoint(buyResaleTicket)]
    fn buy_resale_ticket(&self, ticket_nonce: u64) {
        let payment = self.call_value().egld().clone_value();
        let caller = self.blockchain().get_caller();

        require!(!self.resale_info(ticket_nonce).is_empty(), "Ticket not for sale");
        
        let (seller, festival_id, price) = self.resale_info(ticket_nonce).get();
        require!(payment == price, "Incorrect payment amount");

        // Calculate Tax
        let (tax_norm, tax_sold) = self.festival_tax(festival_id).get();
        let (sold, _inside) = self.festival_state(festival_id).get();
        let (_, _, max) = self.festival_config(festival_id).get();

        let tax_percent = if sold >= max { tax_sold } else { tax_norm };
        let tax_amount = &price * tax_percent as u64 / 100u64;
        let seller_amount = &price - &tax_amount;

        // 1. Pay Seller
        self.send().direct_egld(&seller, &seller_amount);
        // 2. Pay Tax (to contract owner)
        self.send().direct_egld(&self.blockchain().get_owner_address(), &tax_amount);

        // 3. Transfer Ticket to Buyer
        let token = self.ticket_token_identifier().get();
        self.send().direct_esdt(&caller, &token, ticket_nonce, &BigUint::from(1u64));

        // Clear resale info
        self.resale_info(ticket_nonce).clear();
    }

    // ========================================================================
    // EVENTS
    // ========================================================================

    #[event("ticketBought")]
    fn ticket_bought_event(
        &self,
        #[indexed] buyer: &ManagedAddress,
        #[indexed] festival_id: u64,
        #[indexed] ticket_nonce: u64,
    );

    #[event("participantCreated")]
    fn participant_created_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] username: &ManagedBuffer,
    );

    // ========================================================================
    // VIEWS
    // ========================================================================

    #[view(getFestivalData)]
    fn get_festival_data(&self, id: u64) -> (ManagedBuffer, u64, u64, u64, u64, u64) {
        let name = self.festival_name(id).get();
        let (start, end, max) = self.festival_config(id).get();
        let (sold, inside) = self.festival_state(id).get();
        
        (name, start, end, max, sold, inside)
    }

    #[view(getTicketPrices)]
    fn get_ticket_prices_view(&self, festival_id: u64) -> MultiValueEncoded<(ManagedBuffer, ManagedBuffer, BigUint)> {
        let mut result = MultiValueEncoded::new();
        for item in self.ticket_prices(festival_id).iter() {
            result.push(item);
        }
        result
    }

    #[view(getEvents)]
    fn get_events_view(&self, festival_id: u64) -> MultiValueEncoded<(ManagedBuffer, ManagedBuffer, u64, u64)> {
        let mut result = MultiValueEncoded::new();
        for item in self.events(festival_id).iter() {
            result.push(item);
        }
        result
    }
}