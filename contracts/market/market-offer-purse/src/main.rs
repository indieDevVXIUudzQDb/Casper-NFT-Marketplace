#![no_std]
#![no_main]

use casper_contract::{
    contract_api::{
        account::get_main_purse,
        runtime,
        system::{create_purse, transfer_from_purse_to_purse},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{Key, runtime_args, RuntimeArgs, U256, U512};

#[no_mangle]
pub extern "C" fn call() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let item_id: U256 = runtime::get_named_arg("item_id");
    let amount: U512 = runtime::get_named_arg("amount");
    let purse = create_purse();
    transfer_from_purse_to_purse(get_main_purse(), purse, amount, None).unwrap_or_revert();
    runtime::call_contract(
        runtime::get_named_arg("market_contract_hash"),
        "process_market_sale",
        runtime_args! {
            "recipient" => recipient,
            "item_id" => item_id,
            "amount" => amount,
            "market_offer_purse" => purse,
        },
    )
}
