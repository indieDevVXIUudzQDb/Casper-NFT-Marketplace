use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_contract::contract_api::runtime::{print, ret};
use casper_engine_test_support::{
    ARG_AMOUNT, DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG,
    DEFAULT_GENESIS_CONFIG_HASH, DEFAULT_PAYMENT, DEFAULT_RUN_GENESIS_REQUEST,
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder,
};
use casper_execution_engine::core::engine_state::{ExecuteRequest, GenesisAccount};
use casper_execution_engine::core::engine_state::run_genesis_request::RunGenesisRequest;
use casper_types::{
    account::AccountHash, CLTyped, CLValue, ContractHash, ContractPackage, ContractPackageHash,
    HashAddr, Key, Motes, PublicKey, runtime_args, RuntimeArgs, SecretKey, StoredValue,
    U256, U512, URef,
};
use casper_types::account::blake2b;
use casper_types::bytesrepr::{FromBytes, ToBytes};
use casper_types::CLType::ByteArray;
use casper_types::KeyTag::Account;
use cep47_tests::cep47_instance::CEP47Instance;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::error::Error;
use std::hash::Hash;
use std::path::PathBuf;
use test_env::TestEnv;

use crate::market_instance::{MARKET_NAME_KEY, MarketContractInstance, Meta, TokenId};
use crate::market_tests::meta::contract_meta;

const CEP47_NAME: &str = "Dragon NFT";
const CEP47_CONTRACT_NAME: &str = "cep47";
const CEP47_CONTRACT_HASH_KEY: &str = "cep47_contract_hash";
const CEP47_CONTRACT_PACKAGE_HASH_KEY: &str = "cep47_contract_hash_wrapped";
const MARKET_OFFER_PURSE_CONTRACT_HASH_KEY: &str = "market_offer_purse_contract_hash";
const MARKET_OFFER_PURSE_CONTRACT_PACKAGE_HASH_KEY: &str =
    "market_offer_purse_contract_hash_wrapped";
const MARKET_NAME: &str = "Galactic Market";
const MARKET_CONTRACT_NAME: &str = "market";
const MARKET_CONTRACT_HASH_KEY: &str = "market_contract_hash";
const MARKET_CONTRACT_PACKAGE_HASH_KEY: &str = "market_contract_hash_wrapped";
const SYMBOL: &str = "DGNFT";
pub const ITEM_STATUS_AVAILABLE: &str = "available";
pub const ITEM_STATUS_CANCELLED: &str = "cancelled";
pub const ITEM_STATUS_SOLD: &str = "sold";
const MY_ACCOUNT: [u8; 32] = [7u8; 32];
const MARKET_WASM: &str = "market.wasm";
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

pub enum DeploySource {
    Code(PathBuf),
    ByHash { hash: ContractHash, method: String },
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
    path: &[String],
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

pub fn get_contract_hash(
    name: String,
    account: AccountHash,
    test_builder: &InMemoryWasmTestBuilder,
) -> [u8; 32] {
    let key = format!("{}_contract_hash_wrapped", name);
    query(test_builder, Key::Account(account), &[key])
}

pub struct TestAccount {
    secret_key: SecretKey,
    public_key: PublicKey,
    account_hash: AccountHash,
}

pub fn get_test_accounts() -> (InMemoryWasmTestBuilder, Vec<TestAccount>) {
    let mut test_builder = InMemoryWasmTestBuilder::default();
    test_builder
        .run_genesis(&DEFAULT_RUN_GENESIS_REQUEST)
        .commit();

    let mut accounts = Vec::new();
    for i in 0..10u8 {
        let secret_key: SecretKey = SecretKey::ed25519_from_bytes([i; 32]).unwrap();
        let public_key: PublicKey = (&secret_key).into();
        let account_hash = AccountHash::from(&public_key);
        accounts.push(TestAccount {
            secret_key,
            public_key,
            account_hash,
        });
        test_builder
            .exec(fund_account(&account_hash))
            .expect_success()
            .commit();
    }
    (test_builder, accounts)
}

pub struct TestFixture {
    owner: TestAccount,
    cep47_contract_hash: Key,
    cep47_contract_package_hash: Key,
    market_contract_hash: Key,
    market_contract_package_hash: Key,
}

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
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
    dictionary_name: &str,
    dictionary_item_key: String,
) -> Result<StoredValue, String> {
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
                        );
                    }
                };

                let dictionary_uref = named_keys
                    .get(name)
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

fn setup() -> (InMemoryWasmTestBuilder, TestFixture, Vec<TestAccount>) {
    let (mut test_builder, mut accounts) = get_test_accounts();
    let owner = accounts.pop().unwrap();
    let account_address = owner.account_hash;

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

    //get cep47 contract hash
    let cep47_contract_hash = *account
        .named_keys()
        .get(CEP47_CONTRACT_HASH_KEY)
        .expect("should have cep47 contract");

    //get cep47 package hash
    let cep47_contract_package_hash = *account
        .named_keys()
        .get(CEP47_CONTRACT_PACKAGE_HASH_KEY)
        .expect("should have cep47 contract");

    let exec_request = {
        ExecuteRequestBuilder::standard(
            account_address,
            MARKET_WASM,
            runtime_args! {
                "market_name" => MARKET_NAME,
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
    let market_contract_hash = *account
        .named_keys()
        .get(MARKET_CONTRACT_HASH_KEY)
        .expect("should have market contract");

    //get market package hash
    let market_contract_package_hash = *account
        .named_keys()
        // .get(MARKET_CONTRACT_HASH)
        // TODO
        .get(MARKET_CONTRACT_PACKAGE_HASH_KEY)
        .expect("should have market contract");

    // ========= install market contract end========= //

    let test_context = TestFixture {
        owner,
        cep47_contract_hash,
        cep47_contract_package_hash,
        market_contract_hash,
        market_contract_package_hash,
    };

    (test_builder, test_context, accounts)
}

fn nft_mint(
    builder: &mut InMemoryWasmTestBuilder,
    test_context: &TestFixture,
    sender: AccountHash,
    recipient: AccountHash,
    token_ids: Vec<TokenId>,
    token_metas: Vec<Meta>,
) {
    let method: &str = "mint";
    let source = DeploySource::ByHash {
        hash: ContractHash::from(test_context.cep47_contract_hash.into_hash().unwrap()),
        method: method.to_string(),
    };
    let args = runtime_args! {
        "recipient" => Key::Account(recipient),
        "token_ids" => token_ids,
        "token_metas" => token_metas,
    };
    let mut deploy_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_address(sender)
        .with_authorization_keys(&[sender]);
    deploy_builder = match source {
        DeploySource::Code(path) => deploy_builder.with_session_code(path, args),
        DeploySource::ByHash { hash, method } => {
            // let contract_hash = ContractHash::from(*hash);
            deploy_builder.with_stored_session_hash(hash, &*method, args)
        }
    };

    let mut execute_request_builder =
        ExecuteRequestBuilder::from_deploy_item(deploy_builder.build());
    builder
        .exec(execute_request_builder.build())
        .expect_success()
        .commit();
}

fn create_market_item(
    builder: &mut InMemoryWasmTestBuilder,
    test_context: &TestFixture,
    sender: AccountHash,
    recipient: Key,
    item_ids: Vec<TokenId>,
    asking_prices: Vec<U512>,
) {
    let method: &str = "create_market_item";
    let source = DeploySource::ByHash {
        hash: ContractHash::from(test_context.market_contract_hash.into_hash().unwrap()),
        method: method.to_string(),
    };
    let args = runtime_args! {
                "recipient" => recipient,
                "item_ids" => item_ids,
                "item_nft_contract_addresses" => vec![ContractHash::from(test_context.cep47_contract_hash.into_hash().unwrap())],
                "item_asking_prices" => asking_prices,
                "item_token_ids" => vec![TokenId::zero()],
    };
    let mut deploy_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_address(sender)
        .with_authorization_keys(&[sender]);
    deploy_builder = match source {
        DeploySource::Code(path) => deploy_builder.with_session_code(path, args),
        DeploySource::ByHash { hash, method } => {
            // let contract_hash = ContractHash::from(*hash);
            deploy_builder.with_stored_session_hash(hash, &*method, args)
        }
    };

    let mut execute_request_builder =
        ExecuteRequestBuilder::from_deploy_item(deploy_builder.build());
    builder
        .exec(execute_request_builder.build())
        .expect_success()
        .commit();
}

fn process_market_sale(
    builder: &mut InMemoryWasmTestBuilder,
    test_context: &TestFixture,
    recipient: Key,
    sender: AccountHash,
    item_id: TokenId,
    amount: U512,
) {
    let session_code = PathBuf::from("market-offer-purse.wasm");
    let source = DeploySource::Code(session_code);
    let args = runtime_args! {
        "recipient" => recipient,
        "item_id" => item_id,
        "amount" => amount,
        "market_contract_hash" => ContractHash::from(test_context.market_contract_hash.into_hash().unwrap())
    };
    let mut deploy_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_address(sender)
        .with_authorization_keys(&[sender]);
    deploy_builder = match source {
        DeploySource::Code(path) => deploy_builder.with_session_code(path, args),
        DeploySource::ByHash { hash, method } => {
            // let contract_hash = ContractHash::from(*hash);
            deploy_builder.with_stored_session_hash(hash, &*method, args)
        }
    };

    let mut execute_request_builder =
        ExecuteRequestBuilder::from_deploy_item(deploy_builder.build());
    builder
        .exec(execute_request_builder.build())
        .expect_success()
        .commit();
}

fn owner_of(
    builder: &mut InMemoryWasmTestBuilder,
    test_context: &TestFixture,
    token_id: TokenId,
) -> Option<Key> {
    match query_dictionary_item(
        builder,
        test_context.cep47_contract_hash,
        "owners",
        TokenId::zero().to_string(),
    ) {
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

fn get_approved(
    builder: &mut InMemoryWasmTestBuilder,
    test_context: &TestFixture,
    owner: Key,
    token_id: TokenId,
) -> Option<Key> {
    let contract_hash = test_context.cep47_contract_hash;
    let dict_name = "allowances";
    let dictionary_item_key = key_and_value_to_str::<String>(&owner.into(), &token_id.to_string());

    match query_dictionary_item(builder, contract_hash, dict_name, dictionary_item_key) {
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

fn approve(
    builder: &mut InMemoryWasmTestBuilder,
    test_context: &TestFixture,
    sender: AccountHash,
    spender: Key,
    token_ids: Vec<TokenId>,
) {
    let method: &str = "approve";
    let source = DeploySource::ByHash {
        hash: ContractHash::from(test_context.cep47_contract_hash.into_hash().unwrap()),
        method: method.to_string(),
    };
    let args = runtime_args! {
            "spender" => spender,
            "token_ids" => token_ids,
    };
    let mut deploy_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_address(sender)
        .with_authorization_keys(&[sender]);
    deploy_builder = match source {
        DeploySource::Code(path) => deploy_builder.with_session_code(path, args),
        DeploySource::ByHash { hash, method } => {
            // let contract_hash = ContractHash::from(*hash);
            deploy_builder.with_stored_session_hash(hash, &*method, args)
        }
    };

    let mut execute_request_builder =
        ExecuteRequestBuilder::from_deploy_item(deploy_builder.build());
    builder
        .exec(execute_request_builder.build())
        .expect_success()
        .commit();
}

fn transfer_from(
    builder: &mut InMemoryWasmTestBuilder,
    test_context: &TestFixture,
    sender: AccountHash,
    owner: AccountHash,
    recipient: AccountHash,
) {
    let method: &str = "transfer_from";
    let source = DeploySource::ByHash {
        hash: ContractHash::from(test_context.cep47_contract_hash.into_hash().unwrap()),
        method: method.to_string(),
    };
    let args = runtime_args! {
            "sender" => Key::Account(owner),
            "recipient" => Key::Account(recipient),
            "token_ids" => vec![TokenId::zero()],
    };
    let mut deploy_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_address(sender)
        .with_authorization_keys(&[sender]);
    deploy_builder = match source {
        DeploySource::Code(path) => deploy_builder.with_session_code(path, args),
        DeploySource::ByHash { hash, method } => {
            // let contract_hash = ContractHash::from(*hash);
            deploy_builder.with_stored_session_hash(hash, &*method, args)
        }
    };

    let mut execute_request_builder =
        ExecuteRequestBuilder::from_deploy_item(deploy_builder.build());
    builder
        .exec(execute_request_builder.build())
        .expect_success()
        .commit();
}

#[test]
fn should_process_valid_nft_sale() {
    let (mut builder, test_context, mut accounts) = setup();
    let seller = accounts.pop().unwrap();
    let buyer = accounts.pop().unwrap();
    // println!("owner           {:?}", test_context.owner.account_hash);
    // println!("seller          {:?}", &seller.account_hash);
    // println!("buyer           {:?}", &buyer.account_hash);
    // println!(
    //     "cep47_contract_hash     {:?}",
    //     test_context.cep47_contract_hash
    // );
    // println!(
    //     "cep47_contract_package_hash     {:?}",
    //     test_context.cep47_contract_package_hash
    // );
    // println!(
    //     "market_contract_hash     {:?}",
    //     test_context.market_contract_hash
    // );
    // println!(
    //     "market_contract_package_hash     {:?}",
    //     test_context.market_contract_package_hash
    // );

    // Perform mint
    nft_mint(
        &mut builder,
        &test_context,
        test_context.owner.account_hash,
        seller.account_hash,
        vec![TokenId::zero()],
        vec![meta::red_dragon()],
    );

    // Check nft owner
    let owner_before = owner_of(&mut builder, &test_context, TokenId::zero());
    assert_eq!(owner_before.unwrap(), Key::Account(seller.account_hash));

    // --------------- Working --------------- //
    //
    // approve(
    //     &mut builder,
    //     &test_context,
    //     seller.account_hash,
    //     Key::Account(test_context.owner.account_hash),
    //     vec![TokenId::zero()],
    // );
    // let get_approved_result = get_approved(
    //     &mut builder,
    //     &test_context,
    //     Key::Account(seller.account_hash),
    //     TokenId::zero(),
    // );
    // // println!("get_approved_result {:?}", get_approved_result);
    // // assert_eq!(
    // //     get_approved_result.unwrap(),
    // //     Key::Account(test_context.owner.account_hash)
    // // );
    // transfer_from(
    //     &mut builder,
    //     &test_context,
    //     test_context.owner.account_hash,
    //     seller.account_hash,
    //     buyer.account_hash,
    // );
    // let owner_after = owner_of(&mut builder, &test_context, TokenId::zero());
    // assert_eq!(owner_after.unwrap(), Key::Account(buyer.account_hash));

    // --------------- End Working --------------- //

    // --------------- Using contract to transfer --------------- //
    let amount: U512 = 12345.into();

    create_market_item(
        &mut builder,
        &test_context,
        seller.account_hash,
        Key::Account(seller.account_hash),
        vec![TokenId::zero()],
        vec![amount],
    );

    let market_function_hash = builder
        .query(
            None,
            Key::Account(test_context.owner.account_hash),
            // For nested function e.g. process_market_sale
            &[
                MARKET_CONTRACT_HASH_KEY.to_string(),
                "market_item_hash".to_string(),
            ],
            // For the entry level call()
            // &["hello".to_string()],
        )
        .expect("should be stored value.")
        .as_cl_value()
        .expect("should be cl value.")
        .clone()
        .into_t::<Key>()
        .expect("should be key.");
    // println!("market_function_hash {:?}", market_function_hash);

    approve(
        &mut builder,
        &test_context,
        seller.account_hash,
        market_function_hash,
        vec![TokenId::zero()],
    );
    let get_approved_result = get_approved(
        &mut builder,
        &test_context,
        Key::Account(seller.account_hash),
        TokenId::zero(),
    );
    // println!("get_approved_result {:?}", get_approved_result);
    assert_eq!(get_approved_result.unwrap(), market_function_hash);
    process_market_sale(
        &mut builder,
        &test_context,
        Key::Account(buyer.account_hash),
        buyer.account_hash,
        TokenId::zero(),
        amount,
    );
    // TODO check seller account balance

    // // Check nft new owner
    let owner_after = owner_of(&mut builder, &test_context, TokenId::zero());
    assert_eq!(owner_after.unwrap(), Key::Account(buyer.account_hash));
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
