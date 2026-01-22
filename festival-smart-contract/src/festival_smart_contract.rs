#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Essential imports for manual encoding
use multiversx_sc::codec::{TopDecode, TopEncode};

const TICKET_TYPE_FULL: u8 = 0;
const TICKET_TYPE_DAY: u8 = 1;

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

    #[storage_mapper("festivalName")]
    fn festival_name(&self, id: u64) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("festivalConfig")]
    fn festival_config(&self, id: u64) -> SingleValueMapper<(u64, u64, u64)>;

    #[storage_mapper("festivalState")]
    fn festival_state(&self, id: u64) -> SingleValueMapper<(u64, u64)>;

    #[storage_mapper("festivalTax")]
    fn festival_tax(&self, id: u64) -> SingleValueMapper<(u8, u8)>;

    #[storage_mapper("events")]
    fn events(&self, festival_id: u64) -> VecMapper<(ManagedBuffer, ManagedBuffer, u64, u64)>;

    #[storage_mapper("flashEvents")]
    fn flash_events(&self, festival_id: u64) -> VecMapper<(ManagedBuffer, u64, u64, u64)>;

    #[storage_mapper("ticketPrices")]
    fn ticket_prices(
        &self,
        festival_id: u64,
    ) -> VecMapper<(
        ManagedBuffer, // Name
        ManagedBuffer, // Phase
        BigUint,       // Price
        u64,           // Sale Start
        u64,           // Sale End
        u8,            // Ticket Type
    )>;

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

    #[storage_mapper("ticketUsageData")]
    fn ticket_usage_data(&self) -> MapMapper<u64, (ManagedAddress, u64)>;

    #[storage_mapper("products")]
    fn products(
        &self,
        festival_id: u64,
    ) -> VecMapper<(u64, ManagedBuffer, BigUint, ManagedBuffer, ManagedBuffer)>;

    #[storage_mapper("braceletFunds")]
    fn bracelet_funds(&self, festival_id: u64, user: &ManagedAddress)
        -> SingleValueMapper<BigUint>;

    #[storage_mapper("bonusPercentage")]
    fn bonus_percentage(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("egldToUsdRate")]
    fn egld_to_usd_rate(&self) -> SingleValueMapper<BigUint>;

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
        self.festival_config(new_id)
            .set((start_time, end_time, max_tickets));
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
        let (festival_start, festival_end, _max_tickets) = self.festival_config(festival_id).get();
        require!(
            start_time >= festival_start,
            "Event cannot start before the festival"
        );
        require!(
            end_time <= festival_end,
            "Event cannot end after the festival"
        );

        self.events(festival_id)
            .push(&(name, location, start_time, end_time));
    }

    #[only_owner]
    #[endpoint(addTicketPrice)]
    fn add_ticket_price(
        &self,
        festival_id: u64,
        name: ManagedBuffer,
        phase: ManagedBuffer,
        price: BigUint,
        sale_start_time: u64,
        sale_end_time: u64,
        ticket_type: u8,
    ) {
        self.ticket_prices(festival_id).push(&(
            name,
            phase,
            price,
            sale_start_time,
            sale_end_time,
            ticket_type,
        ));
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
        self.flash_events(festival_id)
            .push(&(name, start_time, end_time, bonus_points));
    }

    #[only_owner]
    #[endpoint(setTicketTokenIdentifier)]
    fn set_ticket_token_identifier(&self, token_identifier: TokenIdentifier) {
        self.ticket_token_identifier().set(&token_identifier);
    }

    #[only_owner]
    #[endpoint(addProduct)]
    fn add_product(
        &self,
        festival_id: u64,
        product_id: u64,
        name: ManagedBuffer,
        price: BigUint,
        description: ManagedBuffer,
        image_url: ManagedBuffer,
    ) {
        let product_data = (product_id, name, price, description, image_url);
        self.products(festival_id).push(&product_data);
    }

    #[only_owner]
    #[endpoint(setBonusPercentage)]
    fn set_bonus_percentage(&self, percentage: u64) {
        self.bonus_percentage().set(percentage);
    }

    #[only_owner]
    #[endpoint(setEgldToUsdRate)]
    fn set_egld_to_usd_rate(&self, rate: BigUint) {
        self.egld_to_usd_rate().set(rate);
    }

    #[payable("EGLD")]
    #[endpoint(addFunds)]
    fn add_funds(&self, festival_id: u64) {
        let payment = self.call_value().egld().clone_value();
        require!(payment > 0, "Payment must be positive");
        let caller = self.blockchain().get_caller();

        let rate = self.egld_to_usd_rate().get();
        require!(rate > 0, "EGLD to USD rate not set");

        let mut funds_in_usd = payment * rate;
        let bonus = self.bonus_percentage().get();

        if bonus > 0 {
            let bonus_amount = &funds_in_usd * bonus / 100u64;
            funds_in_usd += bonus_amount;
        }

        self.bracelet_funds(festival_id, &caller)
            .update(|current_funds| *current_funds += funds_in_usd);
    }

    #[payable("EGLD")]
    #[endpoint(buyProduct)]
    fn buy_product(
        &self,
        festival_id: u64,
        product_id: u64,
        quantity: u64,
        pay_with_bracelet: bool,
    ) {
        let caller = self.blockchain().get_caller();
        let mut product_found = false;
        let mut product_price = BigUint::zero();

        for product_tuple in self.products(festival_id).iter() {
            let (p_id, _p_name, p_price, _p_desc, _p_img) = product_tuple;
            if p_id == product_id {
                product_found = true;
                product_price = p_price;
                break;
            }
        }
        require!(product_found, "Product not found");

        let total_price = product_price * quantity;

        if pay_with_bracelet {
            let user_funds = self.bracelet_funds(festival_id, &caller).get();
            require!(user_funds >= total_price, "Insufficient funds in bracelet");

            self.bracelet_funds(festival_id, &caller)
                .update(|current_funds| *current_funds -= &total_price);
        } else {
            let payment = self.call_value().egld().clone_value();
            let rate = self.egld_to_usd_rate().get();
            require!(rate > 0, "EGLD to USD rate not set");

            let payment_in_usd = payment * rate.clone();
            require!(payment_in_usd >= total_price, "Incorrect payment amount");

            if payment_in_usd > total_price {
                let change_in_usd = payment_in_usd - &total_price;
                let change_in_egld = change_in_usd / &rate;
                if change_in_egld > 0 {
                    self.send().direct_egld(&caller, &change_in_egld);
                }
            }
        }
    }

    #[payable("EGLD")]
    #[endpoint(buyTicket)]
    fn buy_ticket(&self, festival_id: u64, ticket_price_name: ManagedBuffer) {
        let payment = self.call_value().egld().clone_value();
        let caller = self.blockchain().get_caller();

        let (_start, _end, max) = self.festival_config(festival_id).get();
        let (sold, inside) = self.festival_state(festival_id).get();
        require!(sold < max, "Tickets are sold out");

        let mut found_price = BigUint::zero();
        let mut ticket_type = TICKET_TYPE_FULL;
        let mut found = false;
        let now = self.blockchain().get_block_timestamp();

        for item in self.ticket_prices(festival_id).iter() {
            let (p_name, _p_phase, p_price, p_sale_start, p_sale_end, p_type) = item;
            if p_name == ticket_price_name {
                require!(
                    now >= p_sale_start && now <= p_sale_end,
                    "Ticket is not available for sale at this time"
                );
                found_price = p_price;
                ticket_type = p_type;
                found = true;
                break;
            }
        }
        require!(found, "Price category not found");
        require!(payment == found_price, "Payment amount is incorrect");

        let token_identifier = self.ticket_token_identifier().get();
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(
            b"https://myfestival.com/ticket.json",
        ));

        // 1. Create a buffer
        let mut attributes_buffer = ManagedBuffer::new();
        // 2. Create the tuple
        let attributes_tuple = (ticket_type, festival_id);
        // 3. Encode the tuple into the buffer (This method is part of TopEncode trait)
        attributes_tuple.top_encode(&mut attributes_buffer).unwrap();

        let festival_name = self.festival_name(festival_id).get();
        let ticket_type_str = if ticket_type == TICKET_TYPE_FULL {
            ManagedBuffer::from("Full Pass")
        } else {
            ManagedBuffer::from("Day Ticket")
        };

        let mut nft_name = festival_name;
        nft_name.append(&ManagedBuffer::from(" - "));
        nft_name.append(&ticket_type_str);

        let new_nonce = self.send().esdt_nft_create(
            &token_identifier,
            &BigUint::from(1u64),
            &nft_name,
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &attributes_buffer,
            &uris,
        );

        self.send()
            .direct_esdt(&caller, &token_identifier, new_nonce, &BigUint::from(1u64));

        self.festival_state(festival_id).set((sold + 1, inside));
        self.ticket_bought_event(&caller, festival_id, new_nonce);
    }

    #[endpoint(createParticipant)]
    fn create_participant(&self, username: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        require!(
            !self.user_list().contains(&caller),
            "Participant already exists"
        );

        self.user_list().insert(caller.clone());
        self.user_name(&caller).set(username.clone());
        self.user_score(&caller).set(0);
        self.user_time_data(&caller).set((0, 0));

        self.participant_created_event(&caller, &username);
    }

    #[only_owner]
    #[endpoint(checkIn)]
    fn check_in(&self, user_address: ManagedAddress, ticket_nonce: u64) {
        let now = self.blockchain().get_block_timestamp();
        let token_identifier = self.ticket_token_identifier().get();

        // 1. Verify ownership
        let balance =
            self.blockchain()
                .get_esdt_balance(&user_address, &token_identifier, ticket_nonce);
        require!(balance == 1, "Ticket not owned by this user");

        // 2. Get token data and attributes
        let token_data =
            self.blockchain()
                .get_esdt_token_data(&user_address, &token_identifier, ticket_nonce);

        let attributes: (u8, u64) = TopDecode::top_decode(token_data.attributes).unwrap();
        let (ticket_type, festival_id) = attributes;

        // 3. Check and update usage data
        if !self.ticket_usage_data().contains_key(&ticket_nonce) {
            self.ticket_usage_data()
                .insert(ticket_nonce, (user_address.clone(), now));
        } else {
            let (first_user, first_check_in_time) =
                self.ticket_usage_data().get(&ticket_nonce).unwrap();
            require!(
                user_address == first_user,
                "Ticket is bound to another user"
            );

            if ticket_type == TICKET_TYPE_DAY {
                require!(
                    now < first_check_in_time + 24 * 3600,
                    "24-hour pass has expired"
                );
            }
        }

        // 4. Update user time data
        let (_last_check_in, total_time) = self.user_time_data(&user_address).get();
        self.user_time_data(&user_address)
            .set((self.blockchain().get_block_timestamp(), total_time));

        // 5. Update festival state
        let (sold, inside) = self.festival_state(festival_id).get();
        self.festival_state(festival_id).set((sold, inside + 1));
    }

    #[only_owner]
    #[endpoint(checkOut)]
    fn check_out(&self, user_address: ManagedAddress, festival_id: u64) {
        require!(
            self.user_list().contains(&user_address),
            "You are not a participant"
        );

        let (last_check_in, total_time) = self.user_time_data(&user_address).get();
        require!(last_check_in > 0, "Not checked in");

        let time_now = self.blockchain().get_block_timestamp();
        let session_time = time_now - last_check_in;
        let new_total_time = total_time + session_time;

        let hours_spent = new_total_time / 3600;
        let current_score = self.user_score(&user_address).get();

        if hours_spent > current_score {
            self.user_score(&user_address).set(hours_spent);
        }

        self.user_time_data(&user_address).set((0, new_total_time));

        let (sold, inside) = self.festival_state(festival_id).get();
        if inside > 0 {
            self.festival_state(festival_id).set((sold, inside - 1));
        }
    }

    #[only_owner]
    #[endpoint(claimFlashEventPoints)]
    fn claim_flash_event_points(
        &self,
        user_address: ManagedAddress,
        festival_id: u64,
        flash_event_index: usize,
    ) {
        require!(
            self.user_list().contains(&user_address),
            "User is not a participant"
        );

        let flash_event = self.flash_events(festival_id).get(flash_event_index);
        let (_name, start, end, bonus) = flash_event;

        let current_time = self.blockchain().get_block_timestamp();
        require!(
            current_time >= start && current_time <= end,
            "Flash event is not active"
        );

        let unique_id = festival_id * 1_000_000 + (flash_event_index as u64);

        require!(
            !self
                .claimed_flash_events(&user_address)
                .contains(&unique_id),
            "Already claimed"
        );

        self.user_score(&user_address)
            .update(|score| *score += bonus);
        self.claimed_flash_events(&user_address).insert(unique_id);
    }

    #[payable("*")]
    #[endpoint(putTicketForSale)]
    fn put_ticket_for_sale(&self, price: BigUint) {
        let (payment_token, payment_nonce, payment_amount) =
            self.call_value().single_esdt().clone().into_tuple();

        require!(
            payment_token == self.ticket_token_identifier().get(),
            "Wrong token sent"
        );
        require!(payment_amount == 1, "Must send exactly 1 ticket");

        require!(
            !self.ticket_usage_data().contains_key(&payment_nonce),
            "Cannot sell a ticket that has been used"
        );

        let caller = self.blockchain().get_caller();

        let token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &payment_token,
            payment_nonce,
        );

        let attributes: (u8, u64) = TopDecode::top_decode(token_data.attributes).unwrap();
        let (_, festival_id) = attributes;

        self.resale_info(payment_nonce)
            .set((caller, festival_id, price));
    }

    #[payable("EGLD")]
    #[endpoint(buyResaleTicket)]
    fn buy_resale_ticket(&self, ticket_nonce: u64) {
        let payment = self.call_value().egld().clone_value();
        let caller = self.blockchain().get_caller();

        require!(
            !self.resale_info(ticket_nonce).is_empty(),
            "Ticket not for sale"
        );

        let (seller, festival_id, price) = self.resale_info(ticket_nonce).get();
        require!(payment == price, "Incorrect payment amount");

        let (tax_norm, tax_sold) = self.festival_tax(festival_id).get();
        let (sold, _inside) = self.festival_state(festival_id).get();
        let (_, _, max) = self.festival_config(festival_id).get();

        let tax_percent = if sold >= max { tax_sold } else { tax_norm };
        let tax_amount = &price * tax_percent as u64 / 100u64;
        let seller_amount = &price - &tax_amount;

        self.send().direct_egld(&seller, &seller_amount);
        self.send()
            .direct_egld(&self.blockchain().get_owner_address(), &tax_amount);

        let token = self.ticket_token_identifier().get();
        self.send()
            .direct_esdt(&caller, &token, ticket_nonce, &BigUint::from(1u64));

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
    fn get_ticket_prices_view(
        &self,
        festival_id: u64,
    ) -> MultiValueEncoded<(ManagedBuffer, ManagedBuffer, BigUint, u64, u64, u8)> {
        let mut result = MultiValueEncoded::new();
        for item in self.ticket_prices(festival_id).iter() {
            result.push(item);
        }
        result
    }

    #[view(getProducts)]
    fn get_products(
        &self,
        festival_id: u64,
    ) -> MultiValueEncoded<(u64, ManagedBuffer, BigUint, ManagedBuffer, ManagedBuffer)> {
        let mut products_result = MultiValueEncoded::new();
        for product in self.products(festival_id).iter() {
            products_result.push(product);
        }
        products_result
    }

    #[view(getBraceletFunds)]

    fn get_bracelet_funds(&self, festival_id: u64, user: ManagedAddress) -> BigUint {
        self.bracelet_funds(festival_id, &user).get()
    }

    #[view(getAllFestivals)]

    fn get_all_festivals(
        &self,
    ) -> MultiValueEncoded<(u64, ManagedBuffer, u64, u64, u64, u64, u64)> {
        let mut festivals = MultiValueEncoded::new();

        let festival_count = self.festival_count().get();

        for id in 1..=festival_count {
            let name = self.festival_name(id).get();

            let (start, end, max) = self.festival_config(id).get();

            let (sold, inside) = self.festival_state(id).get();

            festivals.push((id, name, start, end, max, sold, inside));
        }

        festivals
    }

    #[view(getEventsForFestival)]

    fn get_events_for_festival(
        &self,
        festival_id: u64,
    ) -> MultiValueEncoded<(ManagedBuffer, ManagedBuffer, u64, u64)> {
        let mut result = MultiValueEncoded::new();

        for item in self.events(festival_id).iter() {
            result.push(item);
        }

        result
    }

    #[view(getBonusPercentage)]

    fn get_bonus_percentage(&self) -> u64 {
        self.bonus_percentage().get()
    }

    #[view(getEgldToUsdRate)]

    fn get_egld_to_usd_rate(&self) -> BigUint {
        self.egld_to_usd_rate().get()
    }

    #[view(getLeaderboard)]

    fn get_leaderboard(&self) -> MultiValueEncoded<(ManagedBuffer, u64, u64)> {
        let mut leaderboard = MultiValueEncoded::new();

        for user_address in self.user_list().iter() {
            let username = self.user_name(&user_address).get();

            let score = self.user_score(&user_address).get();

            let (_last_check_in, total_time) = self.user_time_data(&user_address).get();

            leaderboard.push((username, score, total_time));
        }

        leaderboard
    }
}
