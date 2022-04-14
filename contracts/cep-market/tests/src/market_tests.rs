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

use casper_types::account::Account;
use casper_types::CLType::String;
use casper_types::{account::AccountHash, runtime_args, ContractHash, HashAddr, Key, Motes, PublicKey, RuntimeArgs, SecretKey, U256, U512, ContractPackage, CLTyped};
use casper_types::bytesrepr::FromBytes;
use cep47_tests::cep47_instance::CEP47Instance;

use test_env::TestEnv;

use crate::market_instance::{MarketContractInstance, Meta, TokenId, MARKET_NAME_KEY};
const CEP47_NAME: &str = "Dragon NFT";
const CEP47_CONTRACT_NAME: &str = "cep47";
const CEP47_CONTRACT_KEY: &str = "cep47_token_contract";
const CEP47_PACKAGE_KEY: &str = "cep47_contract_hash";
const MARKET_NAME: &str = "Galactic Market";
const MARKET_CONTRACT_NAME: &str = "market";
const MARKET_CONTRACT_KEY: &str = "market_package_contract";
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

fn nft_mint(builder: &mut InMemoryWasmTestBuilder, test_context: &TestFixture){
    let deploy = DeployItemBuilder::new()
        .with_address(test_context.account_address)
        .with_stored_session_named_key(
            CEP47_PACKAGE_KEY,
            "mint",
            runtime_args! {
                "recipient" => Key::Account(test_context.account_address),
                "token_ids" => vec![TokenId::zero()],
                "token_metas" => vec![meta::red_dragon()],
                },
        )
        .with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT, })
        .with_authorization_keys(&[test_context.account_address])
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
                "recipient" => Key::Account(test_context.account_address),
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

#[test]
fn should_process_valid_nft_sale() {

    let (mut builder, test_context) = setup();
    let accounts = get_test_accounts(&mut builder);
    nft_mint(&mut builder, &test_context);
    create_market_item(&mut builder, &test_context);

    let buyer = accounts[0];
    // Check nft owner
    process_market_sale(&mut builder, &test_context, Key::Account(buyer));
    // Check nft new owner

    // // Check nft owner
    // let deploy_item_builder = DeployItemBuilder::new()
    //     .with_empty_payment_bytes(runtime_args! {
    //         ARG_AMOUNT => *DEFAULT_PAYMENT
    //     })
    //     .with_authorization_keys(&[nft_account_addr])
    //     .with_address(nft_account_addr);
    // let deploy_create_store_item = deploy_item_builder.with_stored_session_hash(
    //     nft_contract_hash,
    //     "owner_of",
    //     runtime_args! {
    //             "token_id" => TokenId::zero(),
    //     },
    // ).build();
    // let create_store_item_execute_request =
    //     ExecuteRequestBuilder::from_deploy_item(deploy_create_store_item).build();
    // test_builder.exec(create_store_item_execute_request).commit().expect_success();

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
