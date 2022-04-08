#![no_std]
extern crate alloc;

mod market;
pub mod data;
pub mod event;

pub use market::{Error, MarketContract};
pub use contract_utils;

use casper_types::{ContractHash, U256};
use alloc::{collections::BTreeMap, string::String};

pub type TokenId = U256;
pub type NFTContractAddress = ContractHash;
pub type Meta = BTreeMap<String, String>;

pub const ITEM_STATUS_AVAILABLE: &str= "available";
pub const ITEM_STATUS_CANCELLED: &str= "cancelled";
pub const ITEM_STATUS_SOLD: &str= "sold";
