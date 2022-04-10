use std::collections::BTreeMap;

use casper_types::CLType::String;
use casper_types::{account::AccountHash, ContractHash, HashAddr, Key, PublicKey, U256};
use casper_types::account::Account;
use cep47_tests::cep47_instance::CEP47Instance;

use test_env::TestEnv;

use crate::market_instance::{MarketContractInstance, Meta, TokenId};

const NAME: &str = "DragonsNFT";
const SYMBOL: &str = "DGNFT";
pub const ITEM_STATUS_AVAILABLE: &str = "available";
pub const ITEM_STATUS_CANCELLED: &str = "cancelled";
pub const ITEM_STATUS_SOLD: &str = "sold";

mod meta {
    use super::{BTreeMap, Meta};

    pub fn contract_meta() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("origin".to_string(), "fire".to_string());
        meta
    }

    pub fn red_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "red".to_string());
        meta
    }

    pub fn blue_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "blue".to_string());
        meta
    }

}

fn deploy_nft_contract(env: &TestEnv) -> (CEP47Instance, AccountHash) {
    let nft_contract_owner = env.next_user();
    let nft_contract = CEP47Instance::new(
        &env,
        NAME,
        nft_contract_owner,
        NAME,
        SYMBOL,
        meta::contract_meta(),
    );
    (nft_contract, nft_contract_owner)
}

fn deploy_market_contract(env: &TestEnv, meta: Meta) -> (MarketContractInstance, AccountHash) {
    let market_owner = env.next_user();
    let market_contract = MarketContractInstance::new(&env, NAME, market_owner, NAME, SYMBOL, meta);
    (market_contract, market_owner)
}

#[test]
fn test_mint_one() {
    let env = TestEnv::new();
    let (nft_contract, nft_contract_owner) = deploy_nft_contract(&env);
    let user = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    nft_contract.mint_one(nft_contract_owner, user, token_id, token_meta);
    let first_user_token = nft_contract.get_token_by_index(Key::Account(user), U256::from(0));
    let second_user_token = nft_contract.get_token_by_index(Key::Account(user), U256::from(1));
    assert_eq!(first_user_token, Some(token_id));
    assert_eq!(nft_contract.total_supply(), U256::one());
    assert_eq!(nft_contract.balance_of(Key::Account(user)), U256::one());
    assert_eq!(second_user_token, None);
    assert_eq!(nft_contract.owner_of(token_id).unwrap(), Key::Account(user));
}

#[test]
fn test_deploy() {
    let env = TestEnv::new();
    let (market_contract, _) = deploy_market_contract(&env, meta::contract_meta());
    assert_eq!(market_contract.name(), NAME);
    assert_eq!(market_contract.symbol(), SYMBOL);
    assert_eq!(market_contract.meta(), meta::contract_meta());
    assert_eq!(market_contract.total_supply(), U256::zero());
}

#[test]
fn test_create_market_item() {
    let env = TestEnv::new();
    // Create NFT
    let (nft_contract, nft_contract_owner) = deploy_nft_contract(&env);
    let user = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    nft_contract.mint_one(nft_contract_owner, user, token_id, token_meta);
    let nft_contract_hash = ContractHash::from(nft_contract.contract().contract_hash());
    // Create Market
    let (market_contract, market_owner) = deploy_market_contract(&env, meta::contract_meta());
    let user = env.next_user();
    let item_id = TokenId::from("1");
    market_contract.create_market_item(
        market_owner,
        user,
        item_id,
        nft_contract_hash,
        U256::from("200000"),
        U256::from("1"),
    );

    let first_user_item =
        market_contract.get_owned_item_by_index(Key::Account(user), U256::from(0));
    let second_user_item =
        market_contract.get_owned_item_by_index(Key::Account(user), U256::from(1));
    assert_eq!(first_user_item, Some(item_id));
    assert_eq!(market_contract.meta(), meta::contract_meta());
    assert_eq!(
        market_contract.item_nft_contract_address(item_id).unwrap(),
        nft_contract_hash
    );
    assert_eq!(
        market_contract.item_asking_price(item_id).unwrap(),
        U256::from("200000")
    );
    assert_eq!(
        market_contract.item_token_id(item_id).unwrap(),
        U256::from("1")
    );
    assert_eq!(
        market_contract.item_status(item_id).unwrap(),
        ITEM_STATUS_AVAILABLE
    );
    assert_eq!(market_contract.total_supply(), U256::one());
    assert_eq!(market_contract.balance_of(Key::Account(user)), U256::one());
    assert_eq!(second_user_item, None);
    assert_eq!(
        market_contract.owner_of(item_id).unwrap(),
        Key::Account(user)
    );
}

#[test]
fn test_create_multiple_items() {
    let env = TestEnv::new();
    // Create NFT
    let (nft_contract, nft_contract_owner) = deploy_nft_contract(&env);
    let nft_token_holder = env.next_user();
    let token_id_1 = TokenId::zero();
    let token_id_2 = TokenId::from("1");
    let token_meta_1 = meta::red_dragon();
    let token_meta_2 = meta::blue_dragon();
    nft_contract.mint_one(nft_contract_owner, nft_token_holder, token_id_1, token_meta_1);
    nft_contract.mint_one(nft_contract_owner, nft_token_holder, token_id_2, token_meta_2);
    let nft_contract_hash = ContractHash::from(nft_contract.contract().contract_hash());

    // let my_bytes = [0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8];
    // let nft_contract_hash = ContractHash::new(my_bytes);

    // Create Market
    let (market_contract, market_owner) = deploy_market_contract(&env, meta::contract_meta());
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let item_id_1 = TokenId::from("2");
    let item_id_2 = TokenId::from("3");
    let item_id_3 = TokenId::from("4");

    market_contract.create_market_item(
        market_owner,
        user_1,
        item_id_1.clone(),
        nft_contract_hash.clone(),
        U256::from("200000"),
        U256::from("1"),
    );
    market_contract.create_market_item(
        market_owner,
        user_1,
        item_id_2.clone(),
        nft_contract_hash.clone(),
        U256::from("200000"),
        U256::from("2"),
    );
    market_contract.create_market_item(
        market_owner,
        user_2,
        item_id_3.clone(),
        nft_contract_hash.clone(),
        U256::from("400000"),
        U256::from("3"),
    );



    let user_1_item_1 =
        market_contract.get_owned_item_by_index(Key::Account(user_1), U256::from("0"));
    let user_1_item_2 =
        market_contract.get_owned_item_by_index(Key::Account(user_1), U256::from("1"));
    let user_2_item_1 =
        market_contract.get_owned_item_by_index(Key::Account(user_2), U256::from("0"));

    assert_eq!(user_1_item_1, Some(U256::from("2")));
    assert_eq!(user_1_item_2, Some(U256::from("3")));
    assert_eq!(user_2_item_1, Some(U256::from("4")));

    assert_eq!(
        market_contract.item_token_id(item_id_1).unwrap(),
        U256::from("1")
    );
    assert_eq!(
        market_contract.item_token_id(item_id_2).unwrap(),
        U256::from("2")
    );
    assert_eq!(
        market_contract.item_token_id(item_id_3).unwrap(),
        U256::from("3")
    );
}

#[test]
fn test_sell_market_item() {
    let env = TestEnv::new();
    // Create NFT
    let (nft_contract, nft_contract_owner) = deploy_nft_contract(&env);
    let buyer = env.next_user();
    let seller = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    nft_contract.mint_one(nft_contract_owner, seller, token_id, token_meta);
    let nft_contract_hash = ContractHash::from(nft_contract.contract().contract_hash());
    // Create Market
    let env = TestEnv::new();
    let (market_contract, market_owner) = deploy_market_contract(&env, meta::contract_meta());
    let item_id = TokenId::zero();

    market_contract.create_market_item(
        market_owner,
        seller,
        item_id,
        nft_contract_hash,
        U256::from("200000"),
        U256::zero(),
    );
    let first_user_item =
        market_contract.get_owned_item_by_index(Key::Account(seller), U256::zero());
    assert_eq!(first_user_item, Some(item_id));
    assert_eq!(
        market_contract.item_status(item_id).unwrap(),
        ITEM_STATUS_AVAILABLE
    );
    assert_eq!(nft_contract.owner_of(token_id).unwrap(), Key::Account(seller));
    market_contract.process_market_sale(market_owner, buyer, item_id);
    // assert_eq!(nft_contract.owner_of(token_id).unwrap(), Key::Account(buyer));
    assert_eq!(
        market_contract.item_status(item_id).unwrap(),
        ITEM_STATUS_SOLD
    );
}

#[test]
fn test_should_fail_sell_market_item_insufficient_funds() {}

#[test]
fn test_should_fail_sell_market_item_not_available() {}

#[test]
fn test_cancel_market_item() {}
