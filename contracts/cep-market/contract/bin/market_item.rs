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
use contract::data::{MARKET_NAME, META, SYMBOL};
use contract::{MarketContract, Meta, NFTContractAddress, TokenId};
use contract_utils::{ContractContext, OnChainContractStorage};

#[derive(Default)]
struct MarketItem(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for MarketItem {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl MarketContract<OnChainContractStorage> for MarketItem {}

impl MarketItem {
    fn constructor(&mut self, name: String, symbol: String, meta: Meta) {
        MarketContract::init(self, name, symbol, meta);
    }
}

const KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

#[no_mangle]
fn constructor() {
    let name = runtime::get_named_arg::<String>(MARKET_NAME);
    let symbol = runtime::get_named_arg::<String>(SYMBOL);
    let meta = runtime::get_named_arg::<Meta>(META);
    MarketItem::default().constructor(name, symbol, meta);
}

#[no_mangle]
fn name() {
    let ret = MarketItem::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn symbol() {
    let ret = MarketItem::default().symbol();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn meta() {
    let ret = MarketItem::default().meta();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn total_supply() {
    let ret = MarketItem::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let ret = MarketItem::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_item_by_index() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let index = runtime::get_named_arg::<U256>("index");
    let ret = MarketItem::default().get_item_by_index(owner, index);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn owner_of() {
    let item_id = runtime::get_named_arg::<TokenId>("item_id");
    let ret = MarketItem::default().owner_of(item_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn item_nft_contract_address() {
    let item_id = runtime::get_named_arg::<TokenId>("item_id");
    let ret = MarketItem::default().item_nft_contract_address(item_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn item_asking_price() {
    let item_id = runtime::get_named_arg::<TokenId>("item_id");
    let ret = MarketItem::default().item_asking_price(item_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn item_token_id() {
    let item_id = runtime::get_named_arg::<TokenId>("item_id");
    let ret = MarketItem::default().item_token_id(item_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn item_status() {
    let item_id = runtime::get_named_arg::<TokenId>("item_id");
    let ret = MarketItem::default().item_status(item_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
//
// #[no_mangle]
// fn available_items() {
//     let ret = MarketItem::default().available_items();
//     runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
// }

#[no_mangle]
fn create_market_item() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let item_ids = runtime::get_named_arg::<Vec<TokenId>>("item_ids");
    let item_nft_contract_addresses =
        runtime::get_named_arg::<Vec<NFTContractAddress>>("item_nft_contract_addresses");
    let item_asking_prices = runtime::get_named_arg::<Vec<U256>>("item_asking_prices");
    let item_token_ids = runtime::get_named_arg::<Vec<U256>>("item_token_ids");
    MarketItem::default()
        .create_market_item(
            recipient,
            item_ids,
            item_nft_contract_addresses,
            item_asking_prices,
            item_token_ids,
        )
        .unwrap_or_revert();
}

#[no_mangle]
fn process_market_sale() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let item_id = runtime::get_named_arg::<TokenId>("item_id");
    MarketItem::default()
        .process_market_sale(recipient, item_id)
        .unwrap_or_revert();
}

#[no_mangle]
fn call() {
    // Read arguments for the constructor call.
    let name: String = runtime::get_named_arg(MARKET_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL);
    let meta: Meta = runtime::get_named_arg(META);
    let contract_name: String = runtime::get_named_arg("contract_name");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        MARKET_NAME => name,
        SYMBOL => symbol,
        META => meta
    };

    let (contract_hash, _) = storage::new_contract(
        get_entry_points(),
        None,
        Some(String::from("contract_package_hash")),
        None,
    );

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key("contract_package_hash")
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );

    // The key shouldn't already exist in the named keys.
    let missing_key = runtime::get_key(KEY_NAME);

    // This contract expects a single runtime argument to be provided.  The arg is named "message"
    // and will be of type `String`.
    let value: String = runtime::get_named_arg(RUNTIME_ARG_NAME);

    // Store this value under a new unforgeable reference a.k.a `URef`.
    let value_ref = storage::new_uref(value);

    // Store the new `URef` as a named key with a name of `KEY_NAME`.
    let key = Key::URef(value_ref);
    runtime::put_key(KEY_NAME, key);

    // The key should now be able to be retrieved.  Note that if `get_key()` returns `None`, then
    // `unwrap_or_revert()` will exit the process, returning `ApiError::None`.
    let _retrieved_key = runtime::get_key(KEY_NAME).unwrap_or_revert();
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new(MARKET_NAME, String::cl_type()),
            Parameter::new(SYMBOL, String::cl_type()),
            Parameter::new(META, Meta::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "meta",
        vec![],
        Meta::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "owner_of",
        vec![Parameter::new("item_id", TokenId::cl_type())],
        CLType::Option(Box::new(CLType::Key)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "item_nft_contract_address",
        vec![Parameter::new("item_id", TokenId::cl_type())],
        NFTContractAddress::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "item_asking_price",
        vec![Parameter::new("item_id", TokenId::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "item_token_id",
        vec![Parameter::new("item_id", TokenId::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    // entry_points.add_entry_point(EntryPoint::new(
    //     "available_items",
    //     vec![],
    //     MarketItemList::cl_type(),
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract,
    // ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_market_item",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("item_ids", CLType::List(Box::new(TokenId::cl_type()))),
            Parameter::new(
                "item_nft_contract_addresses",
                CLType::List(Box::new(NFTContractAddress::cl_type())),
            ),
            Parameter::new(
                "item_asking_prices",
                CLType::List(Box::new(U256::cl_type())),
            ),
            Parameter::new("item_token_id", CLType::List(Box::new(U256::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "process_market_sale",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("item_id", TokenId::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_item_by_index",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("index", U256::cl_type()),
        ],
        CLType::Option(Box::new(TokenId::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
