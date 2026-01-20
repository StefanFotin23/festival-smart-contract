#![allow(non_snake_case)]

pub mod config;
mod proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";

pub async fn festival_smart_contract_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let config = Config::new();
    let mut interact = ContractInteract::new(config).await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "upgrade" => interact.upgrade().await,
        "addFestival" => interact.add_festival().await,
        "addEvent" => interact.add_event().await,
        "addTicketPrice" => interact.add_ticket_price().await,
        "addFlashEvent" => interact.add_flash_event().await,
        "setTicketTokenIdentifier" => interact.set_ticket_token_identifier().await,
        "buyTicket" => interact.buy_ticket().await,
        "createParticipant" => interact.create_participant().await,
        "checkIn" => interact.check_in().await,
        "checkOut" => interact.check_out().await,
        "claimFlashEventPoints" => interact.claim_flash_event_points().await,
        "putTicketForSale" => interact.put_ticket_for_sale().await,
        "buyResaleTicket" => interact.buy_resale_ticket().await,
        "getFestivalData" => interact.get_festival_data().await,
        "getTicketPrices" => interact.get_ticket_prices_view().await,
        "getEvents" => interact.get_events_view().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }
    }
    
    impl Drop for State {
        // Serializes state to file
        fn drop(&mut self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }
    }

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("festival-smart-contract");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_all_activations().await;
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/festival-smart-contract.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = new_address.to_bech32_default();
        println!("new address: {new_address_bech32}");
        self.state.set_address(new_address_bech32);
    }

    pub async fn upgrade(&mut self) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .upgrade()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_festival(&mut self) {
        let name = ManagedBuffer::new_from_bytes(&b""[..]);
        let start_time = 0u64;
        let end_time = 0u64;
        let max_tickets = 0u64;
        let tax_normal = 0u8;
        let tax_sold_out = 0u8;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .add_festival(name, start_time, end_time, max_tickets, tax_normal, tax_sold_out)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_event(&mut self) {
        let festival_id = 0u64;
        let name = ManagedBuffer::new_from_bytes(&b""[..]);
        let location = ManagedBuffer::new_from_bytes(&b""[..]);
        let start_time = 0u64;
        let end_time = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .add_event(festival_id, name, location, start_time, end_time)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_ticket_price(&mut self) {
        let festival_id = 0u64;
        let name = ManagedBuffer::new_from_bytes(&b""[..]);
        let phase = ManagedBuffer::new_from_bytes(&b""[..]);
        let price = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .add_ticket_price(festival_id, name, phase, price)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_flash_event(&mut self) {
        let festival_id = 0u64;
        let name = ManagedBuffer::new_from_bytes(&b""[..]);
        let start_time = 0u64;
        let end_time = 0u64;
        let bonus_points = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .add_flash_event(festival_id, name, start_time, end_time, bonus_points)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_ticket_token_identifier(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .set_ticket_token_identifier(token_identifier)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn buy_ticket(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let festival_id = 0u64;
        let ticket_price_name = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .buy_ticket(festival_id, ticket_price_name)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn create_participant(&mut self) {
        let username = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .create_participant(username)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn check_in(&mut self) {
        let festival_id = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .check_in(festival_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn check_out(&mut self) {
        let festival_id = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .check_out(festival_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn claim_flash_event_points(&mut self) {
        let festival_id = 0u64;
        let flash_event_index = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .claim_flash_event_points(festival_id, flash_event_index)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn put_ticket_for_sale(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let festival_id = 0u64;
        let price = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .put_ticket_for_sale(festival_id, price)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn buy_resale_ticket(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let ticket_nonce = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::FestivalSmartContractProxy)
            .buy_resale_ticket(ticket_nonce)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_festival_data(&mut self) {
        let id = 0u64;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FestivalSmartContractProxy)
            .get_festival_data(id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_ticket_prices_view(&mut self) {
        let festival_id = 0u64;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FestivalSmartContractProxy)
            .get_ticket_prices_view(festival_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_events_view(&mut self) {
        let festival_id = 0u64;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FestivalSmartContractProxy)
            .get_events_view(festival_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

}
