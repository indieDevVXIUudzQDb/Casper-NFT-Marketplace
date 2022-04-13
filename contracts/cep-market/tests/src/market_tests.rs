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

const NAME: &str = "DragonsNFT";
const MARKET_NAME: &str = "Market";
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

fn deploy_nft_contract(env: &TestEnv) -> (CEP47Instance, AccountHash) {
    let nft_contract_owner = env.next_user();
    let nft_contract = CEP47Instance::new(
        &env,
        NAME,
        nft_contract_owner,
        NAME,
        SYMBOL,
        meta::contract_meta(),
    );
    (nft_contract, nft_contract_owner)
}

fn deploy_market_contract(env: &TestEnv, meta: Meta) -> (MarketContractInstance, AccountHash) {
    let market_owner = env.next_user();
    let market_contract = MarketContractInstance::new(&env, NAME, market_owner, NAME, SYMBOL, meta);
    (market_contract, market_owner)
}

#[ignore]
#[test]
fn test_mint_one() {
    let env = TestEnv::new();
    let (nft_contract, nft_contract_owner) = deploy_nft_contract(&env);
    let user = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    nft_contract.mint_one(nft_contract_owner, user, token_id, token_meta);
    let first_user_token = nft_contract.get_token_by_index(Key::Account(user), U256::from(0));
    let second_user_token = nft_contract.get_token_by_index(Key::Account(user), U256::from(1));
    assert_eq!(first_user_token, Some(token_id));
    assert_eq!(nft_contract.total_supply(), U256::one());
    assert_eq!(nft_contract.balance_of(Key::Account(user)), U256::one());
    assert_eq!(second_user_token, None);
    assert_eq!(nft_contract.owner_of(token_id).unwrap(), Key::Account(user));
}

#[ignore]
#[test]
fn test_deploy() {
    let env = TestEnv::new();
    let (market_contract, _) = deploy_market_contract(&env, meta::contract_meta());
    assert_eq!(market_contract.name(), NAME);
    assert_eq!(market_contract.symbol(), SYMBOL);
    assert_eq!(market_contract.meta(), meta::contract_meta());
    assert_eq!(market_contract.total_supply(), U256::zero());
}

#[ignore]
#[test]
fn test_create_market_item() {
    let env = TestEnv::new();
    // Create NFT
    let (nft_contract, nft_contract_owner) = deploy_nft_contract(&env);
    let user = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    nft_contract.mint_one(nft_contract_owner, user, token_id, token_meta);
    let nft_contract_hash = ContractHash::from(nft_contract.contract().contract_hash());
    // Create Market
    let (market_contract, market_owner) = deploy_market_contract(&env, meta::contract_meta());
    let user = env.next_user();
    let item_id = TokenId::from("1");
    market_contract.create_market_item(
        market_owner,
        user,
        item_id,
        nft_contract_hash,
        U256::from("200000"),
        U256::from("1"),
    );

    let first_user_item =
        market_contract.get_owned_item_by_index(Key::Account(user), U256::from(0));
    let second_user_item =
        market_contract.get_owned_item_by_index(Key::Account(user), U256::from(1));
    assert_eq!(first_user_item, Some(item_id));
    assert_eq!(market_contract.meta(), meta::contract_meta());
    assert_eq!(
        market_contract.item_nft_contract_address(item_id).unwrap(),
        nft_contract_hash
    );
    assert_eq!(
        market_contract.item_asking_price(item_id).unwrap(),
        U256::from("200000")
    );
    assert_eq!(
        market_contract.item_token_id(item_id).unwrap(),
        U256::from("1")
    );
    assert_eq!(
        market_contract.item_status(item_id).unwrap(),
        ITEM_STATUS_AVAILABLE
    );
    assert_eq!(market_contract.total_supply(), U256::one());
    assert_eq!(market_contract.balance_of(Key::Account(user)), U256::one());
    assert_eq!(second_user_item, None);
    assert_eq!(
        market_contract.owner_of(item_id).unwrap(),
        Key::Account(user)
    );
}

#[ignore]
#[test]
fn test_create_multiple_items() {
    let env = TestEnv::new();
    // Create NFT
    let (nft_contract, nft_contract_owner) = deploy_nft_contract(&env);
    let nft_token_holder = env.next_user();
    let token_id_1 = TokenId::zero();
    let token_id_2 = TokenId::from("1");
    let token_meta_1 = meta::red_dragon();
    let token_meta_2 = meta::blue_dragon();
    nft_contract.mint_one(
        nft_contract_owner,
        nft_token_holder,
        token_id_1,
        token_meta_1,
    );
    nft_contract.mint_one(
        nft_contract_owner,
        nft_token_holder,
        token_id_2,
        token_meta_2,
    );
    let nft_contract_hash = ContractHash::from(nft_contract.contract().contract_hash());

    // let my_bytes = [0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8, 0x1Au8];
    // let nft_contract_hash = ContractHash::new(my_bytes);

    // Create Market
    let (market_contract, market_owner) = deploy_market_contract(&env, meta::contract_meta());
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let item_id_1 = TokenId::from("2");
    let item_id_2 = TokenId::from("3");
    let item_id_3 = TokenId::from("4");

    market_contract.create_market_item(
        market_owner,
        user_1,
        item_id_1.clone(),
        nft_contract_hash.clone(),
        U256::from("200000"),
        U256::from("1"),
    );
    market_contract.create_market_item(
        market_owner,
        user_1,
        item_id_2.clone(),
        nft_contract_hash.clone(),
        U256::from("200000"),
        U256::from("2"),
    );
    market_contract.create_market_item(
        market_owner,
        user_2,
        item_id_3.clone(),
        nft_contract_hash.clone(),
        U256::from("400000"),
        U256::from("3"),
    );

    let user_1_item_1 =
        market_contract.get_owned_item_by_index(Key::Account(user_1), U256::from("0"));
    let user_1_item_2 =
        market_contract.get_owned_item_by_index(Key::Account(user_1), U256::from("1"));
    let user_2_item_1 =
        market_contract.get_owned_item_by_index(Key::Account(user_2), U256::from("0"));

    assert_eq!(user_1_item_1, Some(U256::from("2")));
    assert_eq!(user_1_item_2, Some(U256::from("3")));
    assert_eq!(user_2_item_1, Some(U256::from("4")));

    assert_eq!(
        market_contract.item_token_id(item_id_1).unwrap(),
        U256::from("1")
    );
    assert_eq!(
        market_contract.item_token_id(item_id_2).unwrap(),
        U256::from("2")
    );
    assert_eq!(
        market_contract.item_token_id(item_id_3).unwrap(),
        U256::from("3")
    );
}

#[ignore]
#[test]
fn test_sell_market_item() {
    let env = TestEnv::new();
    // Create NFT
    let (nft_contract, nft_contract_owner) = deploy_nft_contract(&env);
    let buyer = env.next_user();
    let seller = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    nft_contract.mint_one(nft_contract_owner, seller, token_id, token_meta);
    let nft_contract_hash = ContractHash::from(nft_contract.contract().contract_hash());
    // Create Market
    // let env = TestEnv::new();
    let (market_contract, market_owner) = deploy_market_contract(&env, meta::contract_meta());
    let item_id = TokenId::from("1");

    market_contract.create_market_item(
        market_owner,
        seller,
        item_id,
        nft_contract_hash,
        U256::from("200000"),
        U256::zero(),
    );
    let first_user_item =
        market_contract.get_owned_item_by_index(Key::Account(seller), U256::from("1"));
    assert_eq!(first_user_item, Some(item_id));
    assert_eq!(
        market_contract.item_status(item_id).unwrap(),
        ITEM_STATUS_AVAILABLE
    );
    // assert_eq!(nft_contract.owner_of(token_id).unwrap(), Key::Account(seller));
    market_contract.process_market_sale(market_owner, buyer, item_id);
    // assert_eq!(nft_contract.owner_of(token_id).unwrap(), Key::Account(buyer));
    assert_eq!(
        market_contract.item_status(item_id).unwrap(),
        ITEM_STATUS_SOLD
    );
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

pub fn contract_hash(name: std::string::String, account: AccountHash, builder: &InMemoryWasmTestBuilder) -> [u8; 32] {
    let key = format!("{}_contract_hash_wrapped", name);
    // query(builder, Key::Account(account), &[key])
    query(builder, Key::Account(account), &[key])
}

#[test]
fn should_store_hello_world() {
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


    let reciever = accounts[1];

    let nft_deploy_item = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {
            ARG_AMOUNT => *DEFAULT_PAYMENT
        })
        .with_session_code(
            PathBuf::from(NFT_CONTRACT_WASM),
            runtime_args! {
                "name" => NAME,
                "symbol" => SYMBOL,
                "meta" => meta::contract_meta(),
                "contract_name" => NAME,
            },
        )
        .with_authorization_keys(&[account_addr])
        .with_address(account_addr)
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

    let contract_hash_bytes = contract_hash(NAME.to_string(), account_addr, &builder);
    let nft_contract_hash = ContractHash::from(contract_hash_bytes);
    let deploy_mint = deploy_item_builder.with_stored_session_hash(
        nft_contract_hash,
        "mint",
        runtime_args! {
                "recipient" => Key::Account(account_addr),
                "token_ids" => vec![TokenId::from("1")],
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
                MARKET_NAME_KEY => MARKET_NAME,
                "market_symbol" => SYMBOL,
                "market_meta" => meta::contract_meta(),
                "contract_name" => MARKET_NAME,
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

    // Deploy new market item
    let contract_hash_bytes = contract_hash(MARKET_NAME.to_string(), account_addr, &builder);
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
                "item_ids" => vec![TokenId::zero()],
                "item_nft_contract_addresses" => vec![nft_contract_hash],
                "item_asking_prices" => vec![U256::from("2000000")],
                "item_token_ids" => vec![TokenId::from("1")],
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
