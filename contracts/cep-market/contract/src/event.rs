use alloc::vec::Vec;
use casper_types::Key;

use crate::TokenId;

pub enum MarketEvent {
    CreateItem {
        recipient: Key,
        item_ids: Vec<TokenId>,
    },
    Burn {
        owner: Key,
        item_ids: Vec<TokenId>,
    },
    Approve {
        owner: Key,
        spender: Key,
        item_ids: Vec<TokenId>,
    },
    Transfer {
        sender: Key,
        recipient: Key,
        item_ids: Vec<TokenId>,
    }
}
