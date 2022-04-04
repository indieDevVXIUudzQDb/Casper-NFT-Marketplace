use alloc::vec::Vec;
use casper_types::Key;

use crate::TokenId;

pub enum MarketEvent {
    Mint {
        recipient: Key,
        token_ids: Vec<TokenId>,
    }
}
