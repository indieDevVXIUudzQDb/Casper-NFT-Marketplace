#![no_std]
extern crate alloc;

use alloc::{collections::BTreeMap, string::String};
use alloc::vec::Vec;
use casper_types::{CLType, CLTyped, ContractHash, U256};
use casper_types::bytesrepr::{FromBytes, ToBytes};
pub use contract_utils;
pub use market::{Error, MarketContract};

pub mod data;
pub mod event;
mod market;

pub type MarketItemId = U256;
pub type TokenId = U256;
pub type NFTContractAddress = ContractHash;
pub type Meta = BTreeMap<String, String>;

pub const ITEM_STATUS_AVAILABLE: &str = "available";
pub const ITEM_STATUS_CANCELLED: &str = "cancelled";
pub const ITEM_STATUS_SOLD: &str = "sold";

//TODO
// pub struct MarketItemList {
//     ids: Vec<U256>
// }
// impl CLTyped for MarketItemList {
//     fn cl_type() -> CLType {
//         CLType::Any
//     }
// }
// impl FromBytes for MarketItemList {
//     fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
//         let (ids, bytes) = FromBytes::from_bytes(bytes)?;
//         let value = MarketItemList{
//             ids
//         };
//         Ok((value, bytes))
//     }
// }
// impl ToBytes for MarketItemList {
//     fn serialized_length(&self) -> usize {
//         let mut size = 0;
//         size += self.ids.serialized_length();
//         size
//     }
//     fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
//         let mut vec = Vec::with_capacity(self.serialized_length());
//         vec.append(&mut self.ids.to_bytes()?);
//         Ok(vec)
//     }
// }
