use crate::{
    data::{self},
    event::MarketEvent
};
use alloc::{string::String, vec::Vec};
use casper_types::{ApiError};
use contract_utils::{ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    PermissionDenied = 1,
    WrongArguments = 2,
    TokenIdAlreadyExists = 3,
    TokenIdDoesntExist = 4,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait MarketContract<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, name: String) {
        data::set_name(name);
    }
    fn name(&self) -> String {
        data::name()
    }

    fn emit(&mut self, event: MarketEvent) {
        data::emit(&event);
    }
}
