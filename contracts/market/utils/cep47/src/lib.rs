#![no_std]
#[macro_use]
extern crate alloc;

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub use cep47::{CEP47, Error};
pub use contract_utils;

mod cep47;
pub mod data;
pub mod event;

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
