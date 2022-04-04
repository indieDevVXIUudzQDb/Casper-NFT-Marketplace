use std::collections::BTreeMap;

use casper_types::{account::AccountHash, Key, U256};
use test_env::TestEnv;

use crate::market_instance::{MarketContractInstance};

const NAME: &str = "DragonsNFT";
const SYMBOL: &str = "DGNFT";

fn deploy() -> (TestEnv, MarketContractInstance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = MarketContractInstance::new(&env, NAME, owner);
    (env, token, owner)
}

#[test]
fn test_deploy() {
    let (_, token, _) = deploy();
}
