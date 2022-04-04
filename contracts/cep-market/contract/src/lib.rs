#![no_std]
#[macro_use]
extern crate alloc;

mod market;
pub mod data;
pub mod event;

pub use market::{Error, MarketContract};
pub use contract_utils;

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
