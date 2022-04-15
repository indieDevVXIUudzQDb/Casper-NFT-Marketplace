use std::borrow::Borrow;
use casper_contract::contract_api::runtime::print;
use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG,
    DEFAULT_GENESIS_CONFIG_HASH, DEFAULT_PAYMENT,
};
use casper_execution_engine::core::engine_state::run_genesis_request::RunGenesisRequest;
use casper_execution_engine::core::engine_state::{ExecuteRequest, GenesisAccount};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::path::PathBuf;
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::account::Account;
use casper_types::CLType::String;
use casper_types::{account::AccountHash, runtime_args, ContractHash, HashAddr, Key, Motes, PublicKey, RuntimeArgs, SecretKey, U256, U512, ContractPackage, CLTyped, ContractPackageHash, StoredValue};
use casper_types::bytesrepr::{FromBytes, ToBytes};
use cep47_tests::cep47_instance::CEP47Instance;

use test_env::TestEnv;

use crate::market_instance::{MarketContractInstance, Meta, TokenId, MARKET_NAME_KEY};
use crate::market_tests::meta::contract_meta;

const CEP47_NAME: &str = "Dragon NFT";
const CEP47_CONTRACT_NAME: &str = "cep47";
const CEP47_PACKAGE_KEY: &str = "cep47_contract_hash";
const MARKET_NAME: &str = "Galactic Market";
const MARKET_CONTRACT_NAME: &str = "market";
const MARKET_PACKAGE_KEY: &str = "market_contract_hash";
const SYMBOL: &str = "DGNFT";
pub const ITEM_STATUS_AVAILABLE: &str = "available";
pub const ITEM_STATUS_CANCELLED: &str = "cancelled";
pub const ITEM_STATUS_SOLD: &str = "sold";
const MY_ACCOUNT: [u8; 32] = [7u8; 32];
const MARKET_WASM: &str = "contract.wasm";
const CEP47_WASM: &str = "cep47-token.wasm";

mod meta {
    use super::{BTreeMap, Meta};

    pub fn contract_meta() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("origin".to_string(), "fire".to_string());
        meta
    }

    pub fn red_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "red".to_string());
        meta
    }

    pub fn blue_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "blue".to_string());
        meta
    }
}


pub fn fund_account(account: &AccountHash) -> ExecuteRequest {
    let deploy_item = DeployItemBuilder::new()
        .with_address(*DEFAULT_ACCOUNT_ADDR)
        .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_transfer_args(runtime_args! {
            "amount" => U512::from(30_000_000_000_000_u64),
            "target" => *account,
            "id" => <Option::<u64>>::None
        })
        .with_deploy_hash([1; 32])
        .build();

    ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
}

pub fn query<T: FromBytes + CLTyped>(
    test_builder: &InMemoryWasmTestBuilder,
    base: Key,
    path: &[std::string::String],
) -> T {
    test_builder
        .query(None, base, path)
        .expect("should be stored value.")
        .as_cl_value()
        .expect("should be cl value.")
        .clone()
        .into_t()
        .expect("Wrong type in query result.")
}

pub fn get_contract_hash(name: std::string::String, account: AccountHash, test_builder: &InMemoryWasmTestBuilder) -> [u8; 32] {
    let key = format!("{}_contract_hash_wrapped", name);
    // query(test_builder, Key::Account(account), &[key])
    query(test_builder, Key::Account(account), &[key])
}

pub fn get_test_accounts(test_builder: &mut InMemoryWasmTestBuilder) -> Vec<AccountHash> {
    let mut accounts = Vec::new();
    for i in 0..10u8 {
        let secret_key: SecretKey = SecretKey::ed25519_from_bytes([i; 32]).unwrap();
        let public_key: PublicKey = (&secret_key).into();
        let account_hash = AccountHash::from(&public_key);
        accounts.push(account_hash);
        test_builder
            .exec(fund_account(&account_hash))
            .expect_success()
            .commit();
    }
    accounts
}

struct TestFixture {
    account_address: AccountHash,
    cep47_package_hash_key: Key,
    market_package_hash_key: Key
}

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> std::string::String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key.to_bytes().unwrap());
    hasher.update(value.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}

pub fn query_dictionary_item(
    builder: &InMemoryWasmTestBuilder,
    key: Key,
    dictionary_name: std::string::String,
    dictionary_item_key: std::string::String,
) -> Result<StoredValue, std::string::String> {
    let empty_path = vec![];
    let dictionary_key_bytes = dictionary_item_key.as_bytes();
    let address = match key {
        Key::Account(_) | Key::Hash(_) => {
            if let name = dictionary_name {
                let stored_value = builder.query(None, key, &[])?;

                let named_keys = match &stored_value {
                    StoredValue::Account(account) => account.named_keys(),
                    StoredValue::Contract(contract) => contract.named_keys(),
                    _ => {
                        return Err(
                            "Provided base key is nether an account or a contract".to_string()
                        )
                    }
                };

                let dictionary_uref = named_keys
                    .get(&name)
                    .and_then(Key::as_uref)
                    .ok_or_else(|| "No dictionary uref was found in named keys".to_string())?;

                Key::dictionary(*dictionary_uref, dictionary_key_bytes)
            } else {
                return Err("No dictionary name was provided".to_string());
            }
        }
        Key::URef(uref) => Key::dictionary(uref, dictionary_key_bytes),
        Key::Dictionary(address) => Key::Dictionary(address),
        _ => return Err("Unsupported key type for a query to a dictionary item".to_string()),
    };
    builder.query(None, address, &empty_path)
}

fn setup() -> (InMemoryWasmTestBuilder, TestFixture) {

        // Create an asymmetric keypair, and derive the account address of this.
        let secret_key = SecretKey::ed25519_from_bytes([1u8; 32]).unwrap();
        let public_key = PublicKey::from(&secret_key);
        let account_address = AccountHash::from(&public_key);

        // Make this account a genesis account (one which exists at network startup) and a
        // genesis request for the execution engine.
        let account = GenesisAccount::account(
            public_key,
            Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
            None,
        );

        let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
        genesis_config.ee_config_mut().push_account(account);

        let run_genesis_request = RunGenesisRequest::new(
            *DEFAULT_GENESIS_CONFIG_HASH,
            genesis_config.protocol_version(),
            genesis_config.take_ee_config(),
        );

        let mut test_builder = InMemoryWasmTestBuilder::default();
        test_builder.run_genesis(&run_genesis_request).commit();

        // ====install cep47 contract start=========//
        let exec_request = {
            ExecuteRequestBuilder::standard(
                account_address,
                CEP47_WASM,
                runtime_args! {
                "name" => CEP47_NAME,
                "symbol" => SYMBOL,
                "meta" => meta::contract_meta(),
                "contract_name" => CEP47_CONTRACT_NAME,
                },
            )
                .build()
        };

        test_builder.exec(exec_request).expect_success().commit();

        // ======install cep47 contract end =========//

        //get account
        let account = test_builder
            .query(None, Key::Account(account_address), &[])
            .expect("should query account")
            .as_account()
            .cloned()
            .expect("should be account");

        // ========= install market contract start========= //

        //get cep47 package hash
        let cep47_package_hash_key = *account
            .named_keys()
            .get(CEP47_PACKAGE_KEY)
            .expect("should have cep47 contract");

        let exec_request = {
            ExecuteRequestBuilder::standard(
                account_address,
                MARKET_WASM,
                runtime_args! {
                MARKET_NAME_KEY => MARKET_NAME,
                "market_symbol" => SYMBOL,
                "market_meta" => meta::contract_meta(),
                "contract_name" => MARKET_CONTRACT_NAME,
            },
            )
                .build()
        };

        test_builder.exec(exec_request).expect_success().commit();

        // get account
        let account = test_builder
            .query(None, Key::Account(account_address), &[])
            .expect("should query account")
            .as_account()
            .cloned()
            .expect("should be account");


        //get market package hash
        let market_package_hash_key = *account
            .named_keys()
            .get(MARKET_PACKAGE_KEY)
            .expect("should have market contract");

        // ========= install market contract end========= //

    let test_context = TestFixture {
        account_address,
        cep47_package_hash_key,
        market_package_hash_key
    };
    (test_builder, test_context)

}

fn nft_mint(builder: &mut InMemoryWasmTestBuilder, test_context: &TestFixture, sender: AccountHash,recipient:AccountHash, token_ids: Vec<TokenId>, token_metas: Vec<Meta>){
    let deploy = DeployItemBuilder::new()
        .with_address(sender)
        .with_stored_session_named_key(
            CEP47_PACKAGE_KEY,
            "mint",
            runtime_args! {
                "recipient" => Key::Account(recipient),
                "token_ids" => token_ids,
                "token_metas" => token_metas,
                },
        )
        .with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT, })
        .with_authorization_keys(&[sender])
        .with_deploy_hash([42; 32])
        .build();

    let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy).build();
    builder.exec(execute_request).commit().expect_success();
}

fn create_market_item(builder: &mut InMemoryWasmTestBuilder, test_context: &TestFixture){
    let deploy = DeployItemBuilder::new()
        .with_address(test_context.account_address)
        .with_stored_session_named_key(
            MARKET_PACKAGE_KEY,
            "create_market_item",
            runtime_args! {
                "recipient" => Key::Account(test_context.account_address),
                "item_ids" => vec![TokenId::zero()],
                // TODO change to key
                "item_nft_contract_addresses" => vec![ContractHash::from(test_context.cep47_package_hash_key.into_hash().unwrap())],
                "item_asking_prices" => vec![U256::from("2000000")],
                "item_token_ids" => vec![TokenId::zero()],
                },
        )
        .with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT, })
        .with_authorization_keys(&[test_context.account_address])
        .with_deploy_hash([42; 32])
        .build();

    let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy).build();
    builder.exec(execute_request).commit().expect_success();
}

fn process_market_sale(builder: &mut InMemoryWasmTestBuilder, test_context: &TestFixture, buyer: Key){
    let deploy = DeployItemBuilder::new()
        .with_address(test_context.account_address)
        .with_stored_session_named_key(
            MARKET_PACKAGE_KEY,
            "process_market_sale",
            runtime_args! {
                "recipient" => buyer,
                "item_id" => TokenId::zero(),
                },
        )
        .with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT, })
        .with_authorization_keys(&[test_context.account_address])
        .with_deploy_hash([42; 32])
        .build();

    let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy).build();
    builder.exec(execute_request).commit().expect_success();
}

fn owner_of(builder: &mut InMemoryWasmTestBuilder, test_context: &TestFixture, token_id: TokenId) -> Option<Key> {
    match query_dictionary_item(builder, test_context.cep47_package_hash_key, "owners".to_string(), TokenId::zero().to_string()) {
        Ok(value) => value
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t()
            .expect("Wrong type in query result."),
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}

fn get_approved(builder: &mut InMemoryWasmTestBuilder, test_context: &TestFixture,owner: Key, token_id: TokenId) -> Option<Key> {
    match query_dictionary_item(builder, test_context.cep47_package_hash_key, "allowances".to_string(), key_and_value_to_str::<std::string::String>(&owner, &token_id.to_string())) {
        Ok(value) => value
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t()
            .expect("Wrong type in query result."),
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}

fn approve(builder: &mut InMemoryWasmTestBuilder, test_context: &TestFixture, sender: AccountHash, spender: AccountHash){
    let deploy = DeployItemBuilder::new()
        .with_address(test_context.account_address)
        .with_stored_session_named_key(
            CEP47_PACKAGE_KEY,
            "approve",
            runtime_args! {
                "spender" => Key::Account(spender),
                "token_ids" => vec![TokenId::zero()],
                },
        )
        .with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT, })
        .with_authorization_keys(&[test_context.account_address])
        .with_deploy_hash([42; 32])
        .build();

    let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy).build();
    builder.exec(execute_request).commit().expect_success();
}

fn transfer(builder: &mut InMemoryWasmTestBuilder, test_context: &TestFixture, owner:Key, recipient: Key){
    let deploy = DeployItemBuilder::new()
        .with_address(test_context.account_address)
        .with_stored_session_named_key(
            CEP47_PACKAGE_KEY,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "token_ids" => vec![TokenId::zero()],
                },
        )
        .with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT, })
        .with_authorization_keys(&[test_context.account_address])
        .with_deploy_hash([42; 32])
        .build();

    let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy).build();
    builder.exec(execute_request).commit().expect_success();
}

fn transfer_from(builder: &mut InMemoryWasmTestBuilder, test_context: &TestFixture, sender:Key, owner:AccountHash, recipient: Key){
    let deploy = DeployItemBuilder::new()
        .with_address(test_context.account_address)
        .with_stored_session_named_key(
            CEP47_PACKAGE_KEY,
            "transfer_from",
            runtime_args! {
                "sender" => sender,
                "recipient" => recipient,
                "token_ids" => vec![TokenId::zero()],
                },
        )
        .with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT, })
        .with_authorization_keys(&[test_context.account_address])
        .with_deploy_hash([42; 32])
        .build();

    let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy).build();
    builder.exec(execute_request).commit().expect_success();
}

#[test]
fn should_process_valid_nft_sale() {

    let (mut builder, test_context) = setup();
    println!("account_address {:?}", test_context.account_address);
    let accounts = get_test_accounts(&mut builder);
    let original_owner = accounts[0];
    let buyer = accounts[1];

    // --------------- Working --------------- //
    // Perform mint
    nft_mint(&mut builder, &test_context, test_context.account_address, test_context.account_address,vec![TokenId::zero()],vec![meta::red_dragon()] );

    // Check nft owner
    let owner_before = owner_of(&mut builder, &test_context, TokenId::zero());
    assert_eq!(owner_before.unwrap(), Key::Account(test_context.account_address));

    approve(&mut builder, &test_context, test_context.account_address, buyer);
    let approved_after = get_approved(&mut builder, &test_context, Key::Account(buyer),TokenId::zero());
    assert_eq!(approved_after.unwrap(), Key::Account(buyer));


    // --------------- Desired --------------- //
    // // Perform mint
    // nft_mint(&mut builder, &test_context, test_context.account_address, original_owner,vec![TokenId::zero()],vec![meta::red_dragon()] );
    //
    // // Check nft owner
    // let owner_before = owner_of(&mut builder, &test_context, TokenId::zero());
    // assert_eq!(owner_before.unwrap(), Key::Account(original_owner));

    // let approved_before = get_approved(&mut builder, &test_context, Key::Account(original_owner),TokenId::zero());
    // println!("approved_before {:?}", approved_before);
    // approve(&mut builder, &test_context, original_owner, buyer);

    // let approved_after = get_approved(&mut builder, &test_context, Key::Account(original_owner),TokenId::zero());
    // assert_eq!(approved_after.unwrap(), Key::Account(test_context.account_address));
    // println!("approved_after {:?}", approved_after);


    //TODO
    // create_market_item(&mut builder, &test_context);
    // process_market_sale(&mut builder, &test_context, Key::Account(buyer));
    // transfer(&mut builder, &test_context, Key::Account(original_owner),Key::Account(buyer));
    // transfer_from(&mut builder, &test_context, Key::Account(original_owner),Key::Account(test_context.account_address));



    // Check nft new owner
    let owner_after = owner_of(&mut builder, &test_context, TokenId::zero());
    println!("owner_after {:?}", owner_after);

    // assert_ne!(owner_before, owner_after);

}

#[ignore]
#[test]
fn test_should_fail_sell_market_item_insufficient_funds() {}

#[ignore]
#[test]
fn test_should_fail_sell_market_item_not_available() {}

#[ignore]
#[test]
fn test_cancel_market_item() {}
