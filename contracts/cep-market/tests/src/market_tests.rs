use std::collections::BTreeMap;

use casper_types::{account::AccountHash, ContractHash, Key, U256};
use test_env::TestEnv;

use crate::market_instance::{MarketContractInstance, TokenId, Meta};

const NAME: &str = "DragonsNFT";
const SYMBOL: &str = "DGNFT";

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
    // let my_bytes: [u8; 32] = [0x12u8,128];
    let my_bytes = [0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8];
    ContractHash::new(my_bytes)
}


fn deploy(meta: Meta) -> (TestEnv, MarketContractInstance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();


    let token = MarketContractInstance::new(&env, NAME, owner, NAME, SYMBOL, meta );
    (env, token, owner)
}

// #[test]
// fn test_create_market_item() {

#[test]
fn test_deploy() {
    let (_, token, _) = deploy(meta::contract_meta());
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.meta(), meta::contract_meta());
    assert_eq!(token.total_supply(), U256::zero());
}

#[test]
fn test_mint_one() {
    let red_dragon = meta::red_dragon();
    let (env, token, owner) = deploy(red_dragon);
    let user = env.next_user();
    let item_id = TokenId::zero();

    token.mint_one(owner, user, item_id, get_nft_contract_hash(), U256::from("200000"));
    let first_user_token = token.get_item_by_index(Key::Account(user), U256::from(0));
    let second_user_token = token.get_item_by_index(Key::Account(user), U256::from(1));
    assert_eq!(first_user_token, Some(item_id));
    assert_eq!(token.meta(), meta::red_dragon());
    assert_eq!(token.item_nft_contract_address(item_id).unwrap(), get_nft_contract_hash());
    assert_eq!(token.item_asking_price(item_id).unwrap(), U256::from("200000"));
    assert_eq!(token.total_supply(), U256::one());
    assert_eq!(token.balance_of(Key::Account(user)), U256::one());
    assert_eq!(second_user_token, None);
    assert_eq!(token.owner_of(item_id).unwrap(), Key::Account(user));
}
