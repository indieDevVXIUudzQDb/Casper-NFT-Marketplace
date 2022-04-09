use std::collections::BTreeMap;

use casper_types::{account::AccountHash, ContractHash, Key, U256};
use test_env::TestEnv;

use crate::market_instance::{MarketContractInstance, TokenId, Meta};

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
}


fn get_nft_contract_hash() -> ContractHash {
    let my_bytes = [0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8];
    ContractHash::new(my_bytes)
}


fn deploy(meta: Meta) -> (TestEnv, MarketContractInstance, AccountHash) {
    let env = TestEnv::new();
    let market_owner = env.next_user();
    let item = MarketContractInstance::new(&env, NAME, market_owner, NAME, SYMBOL, meta);
    (env, item, market_owner)
}


#[test]
fn test_deploy() {
    let (_, item, _) = deploy(meta::contract_meta());
    assert_eq!(item.name(), NAME);
    assert_eq!(item.symbol(), SYMBOL);
    assert_eq!(item.meta(), meta::contract_meta());
    assert_eq!(item.total_supply(), U256::zero());
}


#[test]
fn test_create_market_item() {
    let red_dragon = meta::red_dragon();
    let (env, item, market_owner) = deploy(red_dragon);
    let user = env.next_user();
    let item_id = TokenId::from("1");

    item.create_market_item(market_owner, user, item_id, get_nft_contract_hash(), U256::from("200000"), U256::from("1"));
    let first_user_item = item.get_owned_item_by_index(Key::Account(user), U256::from(0));
    let second_user_item = item.get_owned_item_by_index(Key::Account(user), U256::from(1));
    assert_eq!(first_user_item, Some(item_id));
    assert_eq!(item.meta(), meta::red_dragon());
    assert_eq!(item.item_nft_contract_address(item_id).unwrap(), get_nft_contract_hash());
    assert_eq!(item.item_asking_price(item_id).unwrap(), U256::from("200000"));
    assert_eq!(item.item_token_id(item_id).unwrap(), U256::from("1"));
    assert_eq!(item.item_status(item_id).unwrap(), ITEM_STATUS_AVAILABLE);
    assert_eq!(item.total_supply(), U256::one());
    assert_eq!(item.balance_of(Key::Account(user)), U256::one());
    assert_eq!(second_user_item, None);
    assert_eq!(item.owner_of(item_id).unwrap(), Key::Account(user));
}

#[test]
fn test_fetch_available_items() {
    let red_dragon = meta::red_dragon();
    let (env, item, market_owner) = deploy(red_dragon);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let item_id_1 = TokenId::zero();
    let item_id_2 = TokenId::from("1");
    let item_id_3 = TokenId::from("2");
    let item_ids = vec![item_id_1, item_id_2];
    let nft_contract_hashes = vec![get_nft_contract_hash(), get_nft_contract_hash()];
    let item_asking_prices = vec![U256::from("200000"), U256::from("300000")];
    let item_token_ids = vec![U256::from("1"), U256::from("2")];
    item.create_market_items(market_owner, user_1, item_ids.clone(), nft_contract_hashes.clone(), item_asking_prices.clone(), item_token_ids.clone());
    item.create_market_item(market_owner, user_2, item_id_3.clone(), get_nft_contract_hash(), U256::from("400000"), U256::from("3"));

    let user_1_item_1 = item.get_owned_item_by_index(Key::Account(user_1), U256::from("0"));
    let user_1_item_2 = item.get_owned_item_by_index(Key::Account(user_1), U256::from("1"));
    let user_2_item_1 = item.get_owned_item_by_index(Key::Account(user_2), U256::from("0"));

    assert_eq!(user_1_item_1, Some(U256::from("0")));
    assert_eq!(user_1_item_2, Some(U256::from("1")));
    assert_eq!(user_2_item_1, Some(U256::from("2")));

    assert_eq!(item.item_token_id(item_id_1).unwrap(), U256::from("1"));
    assert_eq!(item.item_token_id(item_id_2).unwrap(), U256::from("2"));
    assert_eq!(item.item_token_id(item_id_3).unwrap(), U256::from("3"));

    let available_items = item.get_available_items();

    assert_eq!(available_items, Some(U256::from("0")));
    assert_eq!(available_items, Some(U256::from("1")));
    assert_eq!(available_items, Some(U256::from("2")));
}
