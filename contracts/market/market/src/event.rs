use alloc::vec::Vec;
use casper_types::Key;

use crate::MarketItemId;

pub enum MarketEvent {
    CreateItem {
        recipient: Key,
        item_ids: Vec<MarketItemId>,
    },
    SoldItem {
        recipient: Key,
        item_id: MarketItemId,
    },
}
