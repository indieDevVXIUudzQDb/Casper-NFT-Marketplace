use casper_types::{account::AccountHash, ContractHash, Key, U256};
use test_env::TestEnv;

use crate::market_instance::{MarketContractInstance, TokenId};

const NAME: &str = "DragonsNFT";
const SYMBOL: &str = "DGNFT";

fn get_nft_contract_hash() -> ContractHash {
    // let my_bytes: [u8; 32] = [0x12u8,128];
    let my_bytes = [0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8,0x1Au8];
    ContractHash::new(my_bytes)
}


fn deploy() -> (TestEnv, MarketContractInstance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();


    let token = MarketContractInstance::new(&env, NAME, owner, NAME, SYMBOL,
                                            get_nft_contract_hash() );
    (env, token, owner)
}

// #[test]
// fn test_create_market_item() {
//     let (env, token, owner) = deploy();
//     let user = env.next_user();
//     let token_id = TokenId::zero();
//     //TODO create_market_item
//     // let price:i128 = 20000000000;
//     // token.create_market_item(contract_address, owner, user, token_id, price);
//
//     token.mint_one(owner, user, token_id);
//     let first_user_token = token.get_token_by_index(Key::Account(user), U256::from(0 as i32));
//     let second_user_token = token.get_token_by_index(Key::Account(user), U256::from(1 as i32));
//     assert_eq!(first_user_token, Some(token_id));
//     assert_eq!(token.total_supply(), U256::one());
//     assert_eq!(token.balance_of(Key::Account(user)), U256::one());
//     assert_eq!(second_user_token, None);
//     assert_eq!(token.owner_of(token_id).unwrap(), Key::Account(user));
// }

#[test]
fn test_deploy() {
    let (_, token, _) = deploy();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);

    // assert_eq!(token.nft_contract_address(), nft_contract_address::contract_nft_contract_address());
    assert_eq!(token.total_supply(), U256::zero());
}

#[test]
fn test_mint_one() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::zero();
    // let token_nft_contract_address = nft_contract_address::red_dragon();

    token.mint_one(owner, user, token_id, get_nft_contract_hash());
    let first_user_token = token.get_token_by_index(Key::Account(user), U256::from(0));
    let second_user_token = token.get_token_by_index(Key::Account(user), U256::from(1));
    assert_eq!(first_user_token, Some(token_id));
    assert_eq!(token.token_nft_contract_address(token_id).unwrap(), get_nft_contract_hash());
    assert_eq!(token.total_supply(), U256::one());
    assert_eq!(token.balance_of(Key::Account(user)), U256::one());
    assert_eq!(second_user_token, None);
    assert_eq!(token.owner_of(token_id).unwrap(), Key::Account(user));
}
