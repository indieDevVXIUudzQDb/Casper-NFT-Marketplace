use alloc::string::{String, ToString};
use casper_contract::{contract_api::runtime::get_call_stack, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, U256};
use contract_utils::{get_key, key_and_value_to_str, key_to_str, set_key, Dict};

use crate::{event::MarketEvent, Meta, NFTContractAddress, TokenId};

const BALANCES_DICT: &str = "balances";
pub const ALLOWANCES_DICT: &str = "allowances";
const NFT_CONTRACT_ADDRESSES: &str = "nft_contract_addresses";
const ITEM_ASKING_PRICE_DATA: &str = "item_asking_prices";
const ITEM_TOKEN_ID_DATA: &str = "item_token_ids";
const ITEM_STATUS_DATA: &str = "item_statuses";
const OWNERS_DICT: &str = "owners";
const ITEMS_BY_INDEX_DICT: &str = "items_by_index";

const OWNED_TOKENS_BY_INDEX_DICT: &str = "owned_tokens_by_index";
const OWNED_INDEXES_BY_TOKEN_DICT: &str = "owned_indexes_by_token";
pub const NAME: &str = "name";
pub const NFT_CONTRACT_ADDRESS: &str = "meta";
pub const SYMBOL: &str = "symbol";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const META: &str = "meta";

pub struct Owners {
    dict: Dict,
}

impl Owners {
    pub fn instance() -> Owners {
        Owners {
            dict: Dict::instance(OWNERS_DICT),
        }
    }

    pub fn init() {
        Dict::init(OWNERS_DICT)
    }

    pub fn get(&self, key: &TokenId) -> Option<Key> {
        self.dict.get(&key.to_string())
    }

    pub fn set(&self, key: &TokenId, value: Key) {
        self.dict.set(&key.to_string(), value);
    }

    pub fn remove(&self, key: &TokenId) {
        self.dict.remove::<Key>(&key.to_string());
    }
}

pub struct NFTContractAddresses {
    dict: Dict,
}

impl NFTContractAddresses {
    pub fn instance() -> NFTContractAddresses {
        NFTContractAddresses {
            dict: Dict::instance(NFT_CONTRACT_ADDRESSES),
        }
    }

    pub fn init() {
        Dict::init(NFT_CONTRACT_ADDRESSES)
    }

    pub fn get(&self, key: &TokenId) -> Option<NFTContractAddress> {
        self.dict.get(&key.to_string())
    }

    pub fn set(&self, key: &TokenId, value: NFTContractAddress) {
        self.dict.set(&key.to_string(), value);
    }

    pub fn remove(&self, key: &TokenId) {
        self.dict.remove::<NFTContractAddress>(&key.to_string());
    }
}

pub struct ItemAskingPriceData {
    dict: Dict,
}

impl ItemAskingPriceData {
    pub fn instance() -> ItemAskingPriceData {
        ItemAskingPriceData {
            dict: Dict::instance(ITEM_ASKING_PRICE_DATA),
        }
    }

    pub fn init() {
        Dict::init(ITEM_ASKING_PRICE_DATA)
    }

    pub fn get(&self, key: &TokenId) -> Option<U256> {
        self.dict.get(&key.to_string())
    }

    pub fn set(&self, key: &TokenId, value: U256) {
        self.dict.set(&key.to_string(), value);
    }

    pub fn remove(&self, key: &TokenId) {
        self.dict.remove::<U256>(&key.to_string());
    }
}

pub struct ItemTokenIdData {
    dict: Dict,
}

impl ItemTokenIdData {
    pub fn instance() -> ItemTokenIdData {
        ItemTokenIdData {
            dict: Dict::instance(ITEM_TOKEN_ID_DATA),
        }
    }

    pub fn init() {
        Dict::init(ITEM_TOKEN_ID_DATA)
    }

    pub fn get(&self, key: &TokenId) -> Option<U256> {
        self.dict.get(&key.to_string())
    }

    pub fn set(&self, key: &TokenId, value: U256) {
        self.dict.set(&key.to_string(), value);
    }

    pub fn remove(&self, key: &TokenId) {
        self.dict.remove::<U256>(&key.to_string());
    }
}

pub struct ItemStatusData {
    dict: Dict,
}

impl ItemStatusData {
    pub fn instance() -> ItemStatusData {
        ItemStatusData {
            dict: Dict::instance(ITEM_STATUS_DATA),
        }
    }

    pub fn init() {
        Dict::init(ITEM_STATUS_DATA)
    }

    pub fn get(&self, key: &TokenId) -> Option<String> {
        self.dict.get(&key.to_string())
    }

    pub fn set(&self, key: &TokenId, value: String) {
        self.dict.set(&key.to_string(), value);
    }

    pub fn remove(&self, key: &TokenId) {
        self.dict.remove::<String>(&key.to_string());
    }
}

//TODO
// pub struct ItemData {
//     dict: Dict,
// }
// impl ItemData {
//     pub fn instance() -> ItemData {
//         ItemData {
//             dict: Dict::instance(ITEMS_BY_INDEX_DICT),
//         }
//     }
//     pub fn init() {
//         Dict::init(ITEMS_BY_INDEX_DICT)
//     }
//     pub fn get(&self, key: &TokenId) -> Option<String> {
//         self.dict.get(&key.to_string())
//     }
//     pub fn set(&self, key: &TokenId, value: String) {
//         self.dict.set(&key.to_string(), value);
//     }
// }

pub struct OwnedTokens {
    tokens_dict: Dict,
    indexes_dict: Dict,
    balances_dict: Dict,
}

impl OwnedTokens {
    pub fn instance() -> OwnedTokens {
        OwnedTokens {
            tokens_dict: Dict::instance(OWNED_TOKENS_BY_INDEX_DICT),
            indexes_dict: Dict::instance(OWNED_INDEXES_BY_TOKEN_DICT),
            balances_dict: Dict::instance(BALANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(OWNED_TOKENS_BY_INDEX_DICT);
        Dict::init(OWNED_INDEXES_BY_TOKEN_DICT);
        Dict::init(BALANCES_DICT);
    }

    pub fn get_item_by_index(&self, owner: &Key, index: &U256) -> Option<TokenId> {
        self.tokens_dict.get(&key_and_value_to_str(owner, index))
    }

    pub fn get_index_by_token(&self, owner: &Key, value: &TokenId) -> Option<U256> {
        self.indexes_dict.get(&key_and_value_to_str(owner, value))
    }

    pub fn get_balances(&self, owner: &Key) -> U256 {
        self.balances_dict
            .get(&key_to_str(owner))
            .unwrap_or_default()
    }

    pub fn set_balances(&self, owner: &Key, value: U256) {
        self.balances_dict.set(&key_to_str(owner), value);
    }

    pub fn set_token(&self, owner: &Key, value: &TokenId) {
        let length = self.get_balances(owner);
        self.indexes_dict
            .set(&key_and_value_to_str(owner, value), length);
        self.tokens_dict
            .set(&key_and_value_to_str(owner, &length), *value);
        self.set_balances(owner, length + 1);
    }

    pub fn remove_token(&self, owner: &Key, value: &TokenId) {
        let length = self.get_balances(owner);
        let index = self.get_index_by_token(owner, value).unwrap_or_revert();
        match length.cmp(&(index + 1)) {
            core::cmp::Ordering::Equal => {
                self.tokens_dict
                    .remove::<TokenId>(&key_and_value_to_str(owner, &(length - 1)));
                self.set_balances(owner, length - 1);
            }
            core::cmp::Ordering::Greater => {
                let last = self.get_item_by_index(owner, &(length - 1));
                self.indexes_dict.set(
                    &key_and_value_to_str(owner, &last.unwrap_or_revert()),
                    index,
                );
                self.tokens_dict.set(
                    &key_and_value_to_str(owner, &index),
                    last.unwrap_or_revert(),
                );
                self.tokens_dict
                    .remove::<TokenId>(&key_and_value_to_str(owner, &(length - 1)));
                self.set_balances(owner, length - 1);
            }
            core::cmp::Ordering::Less => {}
        }
        self.indexes_dict
            .remove::<U256>(&key_and_value_to_str(owner, value));
    }
}

pub struct Allowances {
    dict: Dict,
}

impl Allowances {
    pub fn instance() -> Allowances {
        Allowances {
            dict: Dict::instance(ALLOWANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(ALLOWANCES_DICT)
    }

    pub fn get(&self, owner: &Key, item_id: &TokenId) -> Option<Key> {
        self.dict
            .get(&key_and_value_to_str::<String>(owner, &item_id.to_string()))
    }

    pub fn set(&self, owner: &Key, item_id: &TokenId, value: Key) {
        self.dict.set(
            &key_and_value_to_str::<String>(owner, &item_id.to_string()),
            value,
        );
    }

    pub fn remove(&self, owner: &Key, item_id: &TokenId) {
        self.dict
            .remove::<Key>(&key_and_value_to_str::<String>(owner, &item_id.to_string()));
    }
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn symbol() -> String {
    get_key(SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}

pub fn meta() -> Meta {
    get_key(META).unwrap_or_revert()
}

pub fn set_meta(meta: Meta) {
    set_key(META, meta);
}

pub fn total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}

pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert();
    let package_hash: Option<ContractPackageHash> = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Some(*contract_package_hash),
        _ => None,
    };
    package_hash.unwrap_or_revert()
}

pub fn emit(_event: &MarketEvent) {}
