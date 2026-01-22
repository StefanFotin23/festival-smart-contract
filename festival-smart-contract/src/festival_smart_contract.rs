#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Essential imports for manual encoding
use multiversx_sc::codec::{TopDecode, TopEncode};

const TICKET_TYPE_FULL: u8 = 0;
const TICKET_TYPE_DAY: u8 = 1;

// Badge types for attendance proof
const BADGE_TYPE_FULL_PASS: u8 = 0;
const BADGE_TYPE_DAY_PASS: u8 = 1;

// Default resale price cap: 300% = 3x original price
const DEFAULT_RESALE_MAX_MULTIPLIER: u64 = 300;

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

    // Stores the original purchase price for each ticket (needed for resale price cap)
    #[storage_mapper("ticketOriginalPrice")]
    fn ticket_original_price(&self, ticket_nonce: u64) -> SingleValueMapper<BigUint>;

    // Maximum resale price multiplier (stored as percentage, e.g., 300 = 3x original price)
    // Default is 300 (3x)
    #[storage_mapper("resaleMaxMultiplier")]
    fn resale_max_multiplier(&self) -> SingleValueMapper<u64>;

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
    ) -> u64 {
        let new_id = self.festival_count().get() + 1;
        self.festival_count().set(new_id);

        self.festival_name(new_id).set(name);
        self.festival_config(new_id)
            .set((start_time, end_time, max_tickets));
        self.festival_state(new_id).set((0, 0));
        self.festival_tax(new_id).set((tax_normal, tax_sold_out));

        new_id
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

    /// Set the maximum resale price multiplier (as percentage)
    /// Example: 300 = 3x original price, 150 = 1.5x original price
    /// Default is 300 (3x) if not set
    #[only_owner]
    #[endpoint(setResaleMaxMultiplier)]
    fn set_resale_max_multiplier(&self, multiplier: u64) {
        require!(multiplier >= 100, "Multiplier must be at least 100 (1x)");
        self.resale_max_multiplier().set(multiplier);
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

        // Store original price for resale price cap enforcement
        self.ticket_original_price(new_nonce).set(&found_price);

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

    /// User-callable check-in: user sends their ticket, receives a badge NFT
    /// This follows the flow:
    /// 1. User sends ticket to contract
    /// 2. Contract validates ticket (not used, valid festival)
    /// 3. Contract marks ticket as used
    /// 4. Contract mints badge NFT and sends it to user
    #[payable("*")]
    #[endpoint(checkIn)]
    fn check_in(&self) {
        let (payment_token, payment_nonce, payment_amount) =
            self.call_value().single_esdt().clone().into_tuple();

        let ticket_token = self.ticket_token_identifier().get();
        require!(payment_token == ticket_token, "Must send a valid ticket");
        require!(payment_amount == 1, "Must send exactly 1 ticket");

        let caller = self.blockchain().get_caller();
        let now = self.blockchain().get_block_timestamp();

        // 1. Get ticket attributes (type and festival_id)
        let token_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &payment_token,
            payment_nonce,
        );

        let attributes: (u8, u64) = TopDecode::top_decode(token_data.attributes).unwrap();
        let (ticket_type, festival_id) = attributes;

        // 2. Verify ticket hasn't been used before (first-time check-in)
        require!(
            !self.ticket_usage_data().contains_key(&payment_nonce),
            "Ticket has already been used for check-in"
        );

        // 3. Festival active check removed for demo purposes
        // In production, uncomment this:
        // let (festival_start, festival_end, _max_tickets) = self.festival_config(festival_id).get();
        // require!(
        //     now >= festival_start && now <= festival_end,
        //     "Festival is not active"
        // );

        // 4. Mark ticket as used
        self.ticket_usage_data()
            .insert(payment_nonce, (caller.clone(), now));

        // 5. Update user time data for check-in
        let (_last_check_in, total_time) = self.user_time_data(&caller).get();
        self.user_time_data(&caller).set((now, total_time));

        // 6. Update festival state (increment people inside)
        let (sold, inside) = self.festival_state(festival_id).get();
        self.festival_state(festival_id).set((sold, inside + 1));

        // 7. Mint and send badge NFT to user (using same collection as tickets)
        let badge_token = self.ticket_token_identifier().get();

        let badge_type = if ticket_type == TICKET_TYPE_FULL {
            BADGE_TYPE_FULL_PASS
        } else {
            BADGE_TYPE_DAY_PASS
        };

        // Create badge attributes
        let mut badge_attributes_buffer = ManagedBuffer::new();
        let badge_attributes = (badge_type, festival_id, now); // type, festival, check-in timestamp
        badge_attributes.top_encode(&mut badge_attributes_buffer).unwrap();

        // Create badge name
        let festival_name = self.festival_name(festival_id).get();
        let badge_type_str = if badge_type == BADGE_TYPE_FULL_PASS {
            ManagedBuffer::from("Full Pass Badge")
        } else {
            ManagedBuffer::from("Day Pass Badge")
        };

        let mut badge_name = festival_name;
        badge_name.append(&ManagedBuffer::from(" - "));
        badge_name.append(&badge_type_str);

        // Create badge URIs
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(
            b"https://myfestival.com/badge.json",
        ));

        // Mint badge NFT
        let badge_nonce = self.send().esdt_nft_create(
            &badge_token,
            &BigUint::from(1u64),
            &badge_name,
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &badge_attributes_buffer,
            &uris,
        );

        // Send badge to user
        self.send()
            .direct_esdt(&caller, &badge_token, badge_nonce, &BigUint::from(1u64));

        // 8. Emit check-in event
        self.check_in_event(&caller, festival_id, payment_nonce, badge_nonce);
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

        // Validate resale price doesn't exceed maximum allowed
        let original_price = self.ticket_original_price(payment_nonce).get();
        require!(
            original_price > 0,
            "Original price not found for this ticket"
        );

        // Get multiplier (default 3x if not set)
        let multiplier = if self.resale_max_multiplier().is_empty() {
            DEFAULT_RESALE_MAX_MULTIPLIER
        } else {
            self.resale_max_multiplier().get()
        };

        // Calculate max allowed price: original_price * multiplier / 100
        let max_price = &original_price * multiplier / 100u64;
        require!(
            price <= max_price,
            "Resale price exceeds maximum allowed (max 3x original price)"
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

    #[event("checkIn")]
    fn check_in_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] festival_id: u64,
        #[indexed] ticket_nonce: u64,
        #[indexed] badge_nonce: u64,
    );

    // ========================================================================
    // VIEWS
    // ========================================================================

        #[view(getFestivalData)]
        fn get_festival_data(&self, id: u64) -> (u64, ManagedBuffer, u64, u64, u64, u64, u64) {
            let name = self.festival_name(id).get();
            let (start, end, max) = self.festival_config(id).get();
            let (sold, inside) = self.festival_state(id).get();
            
            (id, name, start, end, max, sold, inside)
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

    /// Check if a ticket is valid for check-in (owned by user and not used)
    /// Returns: (is_valid, is_used, owner_if_used, check_in_time_if_used)
    #[view(getTicketStatus)]
    fn get_ticket_status(
        &self,
        user_address: ManagedAddress,
        ticket_nonce: u64,
    ) -> (bool, bool, ManagedAddress, u64) {
        let ticket_token = self.ticket_token_identifier().get();

        // Check ownership
        let balance = self
            .blockchain()
            .get_esdt_balance(&user_address, &ticket_token, ticket_nonce);
        let is_owned = balance == 1;

        // Check if used
        if self.ticket_usage_data().contains_key(&ticket_nonce) {
            let (used_by, check_in_time) = self.ticket_usage_data().get(&ticket_nonce).unwrap();
            return (false, true, used_by, check_in_time);
        }

        (is_owned, false, ManagedAddress::zero(), 0)
    }

    /// Get the resale max multiplier (as percentage, e.g., 300 = 3x)
    #[view(getResaleMaxMultiplier)]
    fn get_resale_max_multiplier(&self) -> u64 {
        if self.resale_max_multiplier().is_empty() {
            DEFAULT_RESALE_MAX_MULTIPLIER
        } else {
            self.resale_max_multiplier().get()
        }
    }

    /// Get original purchase price for a ticket
    #[view(getTicketOriginalPrice)]
    fn get_ticket_original_price(&self, ticket_nonce: u64) -> BigUint {
        self.ticket_original_price(ticket_nonce).get()
    }

    /// Get resale info for a ticket including max allowed price
    /// Returns: (is_for_sale, seller, festival_id, asking_price, original_price, max_allowed_price)
    #[view(getResaleInfo)]
    fn get_resale_info(
        &self,
        ticket_nonce: u64,
    ) -> (bool, ManagedAddress, u64, BigUint, BigUint, BigUint) {
        if self.resale_info(ticket_nonce).is_empty() {
            return (
                false,
                ManagedAddress::zero(),
                0,
                BigUint::zero(),
                BigUint::zero(),
                BigUint::zero(),
            );
        }

        let (seller, festival_id, asking_price) = self.resale_info(ticket_nonce).get();
        let original_price = self.ticket_original_price(ticket_nonce).get();
        let multiplier = self.get_resale_max_multiplier();
        let max_allowed_price = &original_price * multiplier / 100u64;

        (
            true,
            seller,
            festival_id,
            asking_price,
            original_price,
            max_allowed_price,
        )
    }

    /// Get all tickets currently for resale for a specific festival
    /// Returns list of: (ticket_nonce, seller, asking_price, original_price)
    #[view(getResaleTicketsForFestival)]
    fn get_resale_tickets_for_festival(
        &self,
        festival_id: u64,
    ) -> MultiValueEncoded<(u64, ManagedAddress, BigUint, BigUint)> {
        let mut result = MultiValueEncoded::new();

        // Note: This requires iterating through all possible nonces
        // In production, you might want a separate set mapper to track resale nonces
        let token = self.ticket_token_identifier().get();
        let sc_address = self.blockchain().get_sc_address();

        // Get the total NFT count from token data (checking nonces 1 to some reasonable max)
        // For now, we'll check nonces up to sold tickets count across all festivals
        let festival_count = self.festival_count().get();
        let mut max_nonce = 0u64;
        for fid in 1..=festival_count {
            let (sold, _) = self.festival_state(fid).get();
            max_nonce += sold;
        }

        for nonce in 1..=max_nonce {
            if !self.resale_info(nonce).is_empty() {
                let (seller, fid, price) = self.resale_info(nonce).get();
                if fid == festival_id {
                    // Verify contract still holds this ticket
                    let balance = self.blockchain().get_esdt_balance(&sc_address, &token, nonce);
                    if balance == 1 {
                        let original_price = self.ticket_original_price(nonce).get();
                        result.push((nonce, seller, price, original_price));
                    }
                }
            }
        }

        result
    }
}
