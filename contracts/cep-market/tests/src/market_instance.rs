use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{account::AccountHash, bytesrepr::ToBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256, ContractHash};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type NFTContractAddress = ContractHash;

pub struct MarketContractInstance(TestContract);

impl MarketContractInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        nft_contract_address: NFTContractAddress,
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
                "nft_contract_address" => nft_contract_address
            },
        ));
        let token_contract_hash = ContractHash::from(instance.0.contract_hash());
        println!("token_contract_hash {}", token_contract_hash);
        instance
    }

    pub fn constructor(&self, sender: AccountHash, name: &str, symbol: &str, nft_contract_address: NFTContractAddress) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "nft_contract_address" => nft_contract_address},
        );
    }

    pub fn mint_one<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        token_id: TokenId,
        token_nft_contract_address: NFTContractAddress,
    ) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "recipient" => recipient.into(),
                "token_ids" => vec![token_id],
                "token_nft_contract_addresses" => vec![token_nft_contract_address]
            },
        )
    }

    pub fn mint_copies<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        token_ids: Vec<TokenId>,
        token_nft_contract_address: NFTContractAddress,
        count: u32,
    ) {
        self.0.call_contract(
            sender,
            "mint_copies",
            runtime_args! {
                "recipient" => recipient.into(),
                "token_ids" => token_ids,
                "token_nft_contract_address" => token_nft_contract_address,
                "count" => count
            },
        )
    }

    pub fn mint_many<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        token_ids: Vec<TokenId>,
        token_nft_contract_addresses: Vec<NFTContractAddress>,
    ) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "recipient" => recipient.into(),
                "token_ids" => token_ids,
                "token_nft_contract_addresses" => token_nft_contract_addresses
            },
        )
    }

    pub fn burn_one<T: Into<Key>>(&self, sender: AccountHash, owner: T, token_id: TokenId) {
        self.0.call_contract(
            sender,
            "burn",
            runtime_args! {
                "owner" => owner.into(),
                "token_ids" => vec![token_id]
            },
        )
    }

    pub fn burn_many<T: Into<Key>>(&self, sender: AccountHash, owner: T, token_ids: Vec<TokenId>) {
        self.0.call_contract(
            sender,
            "burn",
            runtime_args! {
                "owner" => owner.into(),
                "token_ids" => token_ids
            },
        )
    }

    pub fn transfer<T: Into<Key>>(
        &self,
        sender: AccountHash,
        recipient: T,
        token_ids: Vec<TokenId>,
    ) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient.into(),
                "token_ids" => token_ids
            },
        )
    }

    pub fn transfer_from<T: Into<Key>>(
        &self,
        sender: AccountHash,
        owner: T,
        recipient: T,
        token_ids: Vec<TokenId>,
    ) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "sender" => owner.into(),
                "recipient" => recipient.into(),
                "token_ids" => token_ids
            },
        )
    }

    pub fn approve<T: Into<Key>>(&self, sender: AccountHash, spender: T, token_ids: Vec<TokenId>) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {"spender" => spender.into(), "token_ids" => token_ids},
        )
    }

    pub fn get_approved<T: Into<Key>>(&self, owner: T, token_id: TokenId) -> Option<Key> {
        self.0.query_dictionary(
            "allowances",
            key_and_value_to_str::<String>(&owner.into(), &token_id.to_string()),
        )
    }

    pub fn update_token_nft_contract_address(&self, sender: AccountHash, token_id: TokenId, token_nft_contract_address: NFTContractAddress) {
        self.0.call_contract(
            sender,
            "update_token_nft_contract_address",
            runtime_args! {
                "token_id" => token_id,
                "token_nft_contract_address" => token_nft_contract_address
            },
        )
    }

    pub fn get_token_by_index<T: Into<Key>>(&self, account: T, index: U256) -> Option<TokenId> {
        self.0.query_dictionary(
            "owned_tokens_by_index",
            key_and_value_to_str(&account.into(), &index),
        )
    }

    pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn owner_of(&self, token_id: TokenId) -> Option<Key> {
        self.0.query_dictionary("owners", token_id.to_string())
    }

    pub fn token_nft_contract_address(&self, token_id: TokenId) -> Option<NFTContractAddress> {
        self.0.query_dictionary("nft_contract_addresses", token_id.to_string())
    }

    pub fn name(&self) -> String {
        self.0.query_named_key(String::from("name"))
    }

    pub fn symbol(&self) -> String {
        self.0.query_named_key(String::from("symbol"))
    }

    pub fn total_supply(&self) -> U256 {
        self.0.query_named_key(String::from("total_supply"))
    }

    pub fn nft_contract_address(&self) -> Key {
        self.0.query_named_key(String::from("nft_contract_address"))
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
