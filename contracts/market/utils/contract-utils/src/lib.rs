#![no_std]
#![feature(once_cell)]

extern crate alloc;

pub use admin_control::AdminControl;
pub use contract_context::ContractContext;
pub use contract_storage::{ContractStorage, OnChainContractStorage};
pub use data::{Dict, get_key, key_and_value_to_str, key_to_str, set_key};

mod admin_control;
mod contract_context;
mod contract_storage;
mod data;

