use alloc::vec::Vec;
use casper_types::Key;

use crate::TokenId;

pub enum MarketEvent {
    CreateItem {
        recipient: Key,
        item_ids: Vec<TokenId>,
    },
    SoldItem {
        recipient: Key,
        item_id: TokenId,
    }
}
