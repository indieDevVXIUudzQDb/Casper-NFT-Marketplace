use std::collections::BTreeMap;
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{account::AccountHash, bytesrepr::ToBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256, ContractHash};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type NFTContractAddress = ContractHash;
pub type Meta = BTreeMap<String, String>;

const BALANCES_DICT: &str = "balances";
pub const ALLOWANCES_DICT: &str = "allowances";
const NFT_CONTRACT_ADDRESSES: &str = "nft_contract_addresses";
const ITEM_ASKING_PRICE_DATA: &str = "item_asking_prices";
const ITEM_TOKEN_ID_DATA: &str = "item_token_ids";
const ITEM_STATUS_DATA: &str = "item_statuses";
const OWNERS_DICT: &str = "owners";
const OWNED_TOKENS_BY_INDEX_DICT: &str = "owned_tokens_by_index";
// const OWNED_INDEXES_BY_TOKEN_DICT: &str = "owned_indexes_by_token";
pub const NAME: &str = "name";
pub const NFT_CONTRACT_ADDRESS: &str = "meta";
pub const SYMBOL: &str = "symbol";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const META: &str = "meta";


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
        println!("new MarketContractInstance");
        let instance = MarketContractInstance(TestContract::new(
            env,
            "contract.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "meta" => meta
            },
        ));
        let item_contract_hash = ContractHash::from(instance.0.contract_hash());
        println!("item_contract_hash {}", item_contract_hash);
        instance
    }

    pub fn constructor(&self, sender: AccountHash, name: &str, symbol: &str, meta: Meta) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "meta" => meta},
        );
    }

    pub fn create_market_item<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        item_id: TokenId,
        item_nft_contract_address: NFTContractAddress,
        item_asking_price: U256,
        item_token_id: U256,
    ) {
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
    ) {
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

    pub fn get_item_by_index<T: Into<Key>>(&self, account: T, index: U256) -> Option<TokenId> {
        self.0.query_dictionary(
            OWNED_TOKENS_BY_INDEX_DICT,
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
        self.0.query_dictionary(NFT_CONTRACT_ADDRESSES, item_id.to_string())
    }

    pub fn item_asking_price(&self, item_id: TokenId) -> Option<U256> {
        self.0.query_dictionary(ITEM_ASKING_PRICE_DATA, item_id.to_string())
    }

    pub fn item_token_id(&self, item_id: TokenId) -> Option<U256> {
        self.0.query_dictionary(ITEM_TOKEN_ID_DATA, item_id.to_string())
    }

    pub fn item_status(&self, item_id: TokenId) -> Option<String> {
        self.0.query_dictionary(ITEM_STATUS_DATA, item_id.to_string())
    }

    pub fn name(&self) -> String {
        self.0.query_named_key(String::from(NAME))
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
