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

const NFT_CONTRACT_NAME: &str = "DragonsNFT";
const MARKET_CONTRACT_NAME: &str = "Market";
const SYMBOL: &str = "DGNFT";
pub const ITEM_STATUS_AVAILABLE: &str = "available";
pub const ITEM_STATUS_CANCELLED: &str = "cancelled";
pub const ITEM_STATUS_SOLD: &str = "sold";

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

const MY_ACCOUNT: [u8; 32] = [7u8; 32];
// Define `KEY` constant to match that in the contract.
const KEY: &str = "my-key-name";
const VALUE: &str = "hello world";
const RUNTIME_ARG_NAME: &str = "message";
const MARKET_CONTRACT_WASM: &str = "contract.wasm";
const NFT_CONTRACT_WASM: &str = "cep47-token.wasm";

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
    builder: &InMemoryWasmTestBuilder,
    base: Key,
    path: &[std::string::String],
) -> T {
    builder
        .query(None, base, path)
        .expect("should be stored value.")
        .as_cl_value()
        .expect("should be cl value.")
        .clone()
        .into_t()
        .expect("Wrong type in query result.")
}

pub fn get_contract_hash(name: std::string::String, account: AccountHash, builder: &InMemoryWasmTestBuilder) -> [u8; 32] {
    let key = format!("{}_contract_hash_wrapped", name);
    // query(builder, Key::Account(account), &[key])
    query(builder, Key::Account(account), &[key])
}

#[test]
fn should_process_valid_nft_sale() {
    // Create keypair.
    let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
    let public_key = PublicKey::from(&secret_key);

    // Create an AccountHash from a public key.
    let account_addr = AccountHash::from(&public_key);
    // Create a GenesisAccount.
    let account = GenesisAccount::account(
        public_key,
        Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
        None,
    );




    let mut builder = InMemoryWasmTestBuilder::default();
    let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
    genesis_config.ee_config_mut().push_account(account);
    let run_genesis_request = RunGenesisRequest::new(
        *DEFAULT_GENESIS_CONFIG_HASH,
        genesis_config.protocol_version(),
        genesis_config.take_ee_config(),
    );
    builder.run_genesis(&run_genesis_request).commit();

    let mut accounts = Vec::new();
    for i in 0..10u8 {
        let secret_key: SecretKey = SecretKey::ed25519_from_bytes([i; 32]).unwrap();
        let public_key: PublicKey = (&secret_key).into();
        let account_hash = AccountHash::from(&public_key);
        accounts.push(account_hash);
        builder
            .exec(fund_account(&account_hash))
            .expect_success()
            .commit();
    }

    let nft_account_addr = accounts[0];

    let reciever = accounts[1];

    let nft_deploy_item = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {
            ARG_AMOUNT => *DEFAULT_PAYMENT
        })
        .with_session_code(
            PathBuf::from(NFT_CONTRACT_WASM),
            runtime_args! {
                "name" => NFT_CONTRACT_NAME,
                "symbol" => SYMBOL,
                "meta" => meta::contract_meta(),
                "contract_name" => NFT_CONTRACT_NAME,
            },
        )
        .with_authorization_keys(&[nft_account_addr])
        .with_address(nft_account_addr)
        .build();

    let nft_execute_request = ExecuteRequestBuilder::from_deploy_item(nft_deploy_item).build();

    // deploy NFT contract.
    builder.exec(nft_execute_request).commit().expect_success();


    let deploy_item_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {
            ARG_AMOUNT => *DEFAULT_PAYMENT
        })
        .with_authorization_keys(&[account_addr])
        .with_address(account_addr);

    let contract_hash_bytes = get_contract_hash(NFT_CONTRACT_NAME.to_string(), nft_account_addr, &builder);
    let nft_contract_hash = ContractHash::from(contract_hash_bytes);
    let deploy_mint = deploy_item_builder.with_stored_session_hash(
        nft_contract_hash,
        "mint",
        runtime_args! {
                "recipient" => Key::Account(nft_account_addr),
                "token_ids" => vec![TokenId::zero()],
                "token_metas" => vec![meta::red_dragon()],
        },
    ).build();
    let nft_mint_execute_request =
        ExecuteRequestBuilder::from_deploy_item(deploy_mint).build();
    builder.exec(nft_mint_execute_request).commit().expect_success();


    let market_deploy_item = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {
            ARG_AMOUNT => *DEFAULT_PAYMENT
        })
        .with_session_code(
            PathBuf::from(MARKET_CONTRACT_WASM),
            runtime_args! {
                MARKET_NAME_KEY => MARKET_CONTRACT_NAME,
                "market_symbol" => SYMBOL,
                "market_meta" => meta::contract_meta(),
                "contract_name" => MARKET_CONTRACT_NAME,
                RUNTIME_ARG_NAME => VALUE,
            },
        )
        .with_authorization_keys(&[account_addr])
        .with_address(account_addr)
        .build();

    let market_execute_request =
        ExecuteRequestBuilder::from_deploy_item(market_deploy_item).build();


    // prepare assertions.
    let result_of_query = builder.query(
        None,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        &[KEY.to_string()],
    );
    assert!(result_of_query.is_err());
    builder
        .exec(market_execute_request)
        .commit()
        .expect_success();


    // make assertions
    let result_of_query = builder
        .query(None, Key::Account(account_addr), &[KEY.to_string()])
        .expect("should be stored value.")
        .as_cl_value()
        .expect("should be cl value.")
        .clone()
        .into_t::<std::string::String>()
        .expect("should be string.");

    assert_eq!(result_of_query, VALUE);

    // Create market item
    let contract_hash_bytes = get_contract_hash(MARKET_CONTRACT_NAME.to_string(), account_addr, &builder);
    let market_contract_hash = ContractHash::from(contract_hash_bytes);
    let deploy_item_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {
            ARG_AMOUNT => *DEFAULT_PAYMENT
        })
        .with_authorization_keys(&[account_addr])
        .with_address(account_addr);
    let deploy_create_store_item = deploy_item_builder.with_stored_session_hash(
        market_contract_hash,
        "create_market_item",
        runtime_args! {
                "recipient" => Key::Account(account_addr),
                "item_ids" => vec![TokenId::from("1")],
                "item_nft_contract_addresses" => vec![nft_contract_hash],
                "item_asking_prices" => vec![U256::from("2000000")],
                "item_token_ids" => vec![TokenId::zero()],
        },
    ).build();
    let create_store_item_execute_request =
        ExecuteRequestBuilder::from_deploy_item(deploy_create_store_item).build();
    builder.exec(create_store_item_execute_request).commit().expect_success();


    // Check nft owner
    let deploy_item_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {
            ARG_AMOUNT => *DEFAULT_PAYMENT
        })
        .with_authorization_keys(&[nft_account_addr])
        .with_address(nft_account_addr);
    let deploy_create_store_item = deploy_item_builder.with_stored_session_hash(
        nft_contract_hash,
        "owner_of",
        runtime_args! {
                "token_id" => TokenId::zero(),
        },
    ).build();
    let create_store_item_execute_request =
        ExecuteRequestBuilder::from_deploy_item(deploy_create_store_item).build();
    builder.exec(create_store_item_execute_request).commit().expect_success();

    // Process market sale
    let deploy_item_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {
            ARG_AMOUNT => *DEFAULT_PAYMENT
        })
        .with_authorization_keys(&[account_addr])
        .with_address(account_addr);
    let deploy_create_store_item = deploy_item_builder.with_stored_session_hash(
        market_contract_hash,
        "process_market_sale",
        runtime_args! {
                "recipient" => Key::Account(account_addr),
                "item_id" => TokenId::zero(),
        },
    ).build();
    let create_store_item_execute_request =
        ExecuteRequestBuilder::from_deploy_item(deploy_create_store_item).build();
    builder.exec(create_store_item_execute_request).commit().expect_success();
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
