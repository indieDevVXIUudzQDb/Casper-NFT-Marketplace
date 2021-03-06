use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::WasmTestBuilder;
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, CLTyped, ContractHash, Key, runtime_args,
    RuntimeArgs, U256, U512,
};
use std::collections::BTreeMap;
use std::fmt::Debug;
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type NFTContractAddress = ContractHash;
pub type Meta = BTreeMap<String, String>;

const BALANCES_DICT: &str = "item_balances";
pub const ALLOWANCES_DICT: &str = "item_allowances";
const NFT_CONTRACT_ADDRESSES: &str = "nft_contract_addresses";
const ITEM_ASKING_PRICE_DATA: &str = "item_asking_prices";
const ITEM_TOKEN_ID_DATA: &str = "item_token_ids";
const ITEM_STATUS_DATA: &str = "item_statuses";
const ITEM_PURSE_DATA: &str = "item_purses";
const OWNERS_DICT: &str = "item_owners";
const OWNED_ITEMS_BY_INDEX_DICT: &str = "owned_items_by_index";
const OWNED_INDEXES_BY_ITEM_DICT: &str = "owned_indexes_by_item";
pub const NFT_CONTRACT_ADDRESS: &str = "nft_contract_hash";
pub const TOTAL_SUPPLY: &str = "item_total_supply";
pub const MARKET_NAME_KEY: &str = "market_name";
pub const SYMBOL: &str = "market_symbol";
pub const META: &str = "market_meta";

pub struct MarketContractInstance(TestContract);

impl MarketContractInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        meta: Meta,
    ) -> MarketContractInstance {
        let instance = MarketContractInstance(TestContract::new(
            env,
            "market.wasm",
            contract_name,
            sender,
            runtime_args! {
                MARKET_NAME_KEY => name,
                SYMBOL => symbol,
                META => meta
            },
        ));
        instance
    }

    pub fn constructor(&self, sender: AccountHash, name: &str, symbol: &str, meta: Meta) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
            MARKET_NAME_KEY => name,
            SYMBOL => symbol,
            META => meta},
        );
    }

    pub fn create_market_item<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        item_id: TokenId,
        item_nft_contract_address: NFTContractAddress,
        item_asking_price: U512,
        item_token_id: U256,
    ) -> WasmTestBuilder<InMemoryGlobalState> {
        self.0.call_contract(
            sender,
            "create_market_item",
            runtime_args! {
                "recipient" => recipient.into(),
                "item_ids" => vec![item_id],
                "item_nft_contract_addresses" => vec![item_nft_contract_address],
                "item_asking_prices" => vec![item_asking_price],
                "item_token_ids" => vec![item_token_id]
            },
        )
    }

    pub fn create_market_items<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        item_ids: Vec<TokenId>,
        item_nft_contract_addresses: Vec<NFTContractAddress>,
        item_asking_prices: Vec<U256>,
        item_token_ids: Vec<U256>,
    ) -> WasmTestBuilder<InMemoryGlobalState> {
        self.0.call_contract(
            sender,
            "create_market_item",
            runtime_args! {
                "recipient" => recipient.into(),
                "item_ids" => item_ids,
                "item_nft_contract_addresses" => item_nft_contract_addresses,
                "item_asking_prices" => item_asking_prices,
                "item_token_ids" => item_token_ids
            },
        )
    }

    pub fn process_market_sale<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        item_id: TokenId,
    ) -> WasmTestBuilder<InMemoryGlobalState> {
        self.0.call_contract(
            sender,
            "process_market_sale",
            runtime_args! {
                "recipient" => recipient.into(),
                "item_id" => item_id,
            },
        )
    }

    pub fn get_available_items<T: Into<Key>>(
        &self,
        sender: AccountHash,
    ) -> WasmTestBuilder<InMemoryGlobalState> {
        self.0
            .call_contract(sender, "get_available_items", runtime_args! {})
    }

    pub fn get_owned_item_by_index<T: Into<Key>>(
        &self,
        account: T,
        index: U256,
    ) -> Option<TokenId> {
        self.0.query_dictionary(
            OWNED_ITEMS_BY_INDEX_DICT,
            key_and_value_to_str(&account.into(), &index),
        )
    }

    pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary(BALANCES_DICT, key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn owner_of(&self, item_id: TokenId) -> Option<Key> {
        self.0.query_dictionary(OWNERS_DICT, item_id.to_string())
    }

    pub fn item_nft_contract_address(&self, item_id: TokenId) -> Option<NFTContractAddress> {
        self.0
            .query_dictionary(NFT_CONTRACT_ADDRESSES, item_id.to_string())
    }

    pub fn item_asking_price(&self, item_id: TokenId) -> Option<U512> {
        self.0
            .query_dictionary(ITEM_ASKING_PRICE_DATA, item_id.to_string())
    }

    pub fn item_token_id(&self, item_id: TokenId) -> Option<U256> {
        self.0
            .query_dictionary(ITEM_TOKEN_ID_DATA, item_id.to_string())
    }

    pub fn item_status(&self, item_id: TokenId) -> Option<String> {
        self.0
            .query_dictionary(ITEM_STATUS_DATA, item_id.to_string())
    }

    pub fn name(&self) -> String {
        self.0.query_named_key(String::from(MARKET_NAME_KEY))
    }

    pub fn symbol(&self) -> String {
        self.0.query_named_key(String::from(SYMBOL))
    }

    pub fn total_supply(&self) -> U256 {
        self.0.query_named_key(String::from(TOTAL_SUPPLY))
    }

    pub fn meta(&self) -> Meta {
        self.0.query_named_key(String::from(META))
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key.to_bytes().unwrap());
    hasher.update(value.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
