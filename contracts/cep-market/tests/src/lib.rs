use casper_types::bytesrepr::{FromBytes, ToBytes};
use casper_types::{CLType, CLTyped, U256};
use std::collections::BTreeMap;

#[cfg(test)]
pub mod market_tests;

#[cfg(test)]
pub mod market_instance;

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
