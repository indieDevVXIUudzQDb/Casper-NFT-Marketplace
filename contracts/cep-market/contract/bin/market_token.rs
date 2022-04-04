#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract::{Meta, TokenId, MarketContract};
use contract_utils::{ContractContext, OnChainContractStorage};

#[derive(Default)]
struct NFTToken(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for NFTToken {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl MarketContract<OnChainContractStorage> for NFTToken {}
impl NFTToken {
    fn constructor(&mut self, name: String) {
        MarketContract::init(self, name);
    }
}

#[no_mangle]
fn constructor() {
    let name = runtime::get_named_arg::<String>("name");
    NFTToken::default().constructor(name);
}

#[no_mangle]
fn name() {
    let ret = NFTToken::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn call() {
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points
}
