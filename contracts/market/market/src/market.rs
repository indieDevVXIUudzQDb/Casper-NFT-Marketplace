use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use casper_contract::contract_api::{runtime, storage, system};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ApiError, Key, runtime_args, RuntimeArgs, U256, U512, URef};
use contract_utils::{ContractContext, ContractStorage};
use core::convert::TryInto;

use crate::{data::{self}, event::MarketEvent, ITEM_STATUS_AVAILABLE, ITEM_STATUS_SOLD, Meta, NFTContractAddress, MarketItemId, TokenId};
use crate::data::{Allowances, ItemAskingPriceData, ItemStatusData, ItemTokenIdData, NFTContractAddresses, NFTMarketItemIds, OwnedTokens, Owners};

#[repr(u16)]
pub enum Error {
    PermissionDenied = 1,
    WrongArguments = 2,
    MarketItemIdAlreadyExists = 3,
    MarketItemIdDoesntExist = 4,
    MarketItemNotAvailable = 5,
    BalanceNotFound = 6,
    BalanceMismatch,
}

const METHOD_BALANCE: &str = "balance";
const ARG_PURSE: &str = "purse";

macro_rules! zip {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        $x.iter().zip(
            zip!($($y), +))
    )
}


impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait MarketContract<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, name: String, symbol: String, meta: Meta) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_meta(meta);
        data::set_total_supply(U256::zero());
        Owners::init();
        OwnedTokens::init();
        NFTContractAddresses::init();
        NFTMarketItemIds::init();
        ItemAskingPriceData::init();
        ItemStatusData::init();
        ItemTokenIdData::init();
        Allowances::init();
        let contract_hash = Key::Hash(self.self_addr().into_hash().unwrap());
        let value_ref = storage::new_uref(contract_hash);
        // TODO improve naming for this
        runtime::put_key("market_item_hash", Key::URef(value_ref));
    }

    fn name(&self) -> String {
        data::name()
    }

    fn symbol(&self) -> String {
        data::symbol()
    }

    fn meta(&self) -> Meta {
        data::meta()
    }

    fn total_supply(&self) -> U256 {
        data::total_supply()
    }

    fn balance_of(&self, owner: Key) -> U256 {
        OwnedTokens::instance().get_balances(&owner)
    }

    fn owner_of(&self, item_id: MarketItemId) -> Option<Key> {
        Owners::instance().get(&item_id)
    }

    fn item_nft_contract_address(&self, item_id: MarketItemId) -> Option<NFTContractAddress> {
        NFTContractAddresses::instance().get(&item_id)
    }

    fn token_market_status(&self, item_token_id: U256) -> Option<String> {
        let market_item_ids = NFTMarketItemIds::instance().get(item_token_id).unwrap();
        self.item_status(*market_item_ids.last().unwrap())
    }

    fn set_item_nft_contract_address(
        &mut self,
        item_id: MarketItemId,
        nft_contract_hash: NFTContractAddress,
    ) -> Result<(), Error> {
        if self.owner_of(item_id).is_none() {
            return Err(Error::MarketItemIdDoesntExist);
        };

        let nft_contract_addresses_dict = NFTContractAddresses::instance();
        nft_contract_addresses_dict.set(&item_id, nft_contract_hash);

        Ok(())
    }

    fn item_asking_price(&self, item_id: MarketItemId) -> Option<U512> {
        ItemAskingPriceData::instance().get(&item_id)
    }

    fn item_token_id(&self, item_id: MarketItemId) -> Option<U256> {
        ItemTokenIdData::instance().get(&item_id)
    }

    fn item_status(&self, item_id: MarketItemId) -> Option<String> {
        ItemStatusData::instance().get(&item_id)
    }

    fn set_item_status(&mut self, item_id: MarketItemId, value: String) -> Result<(), Error> {
        if self.owner_of(item_id).is_none() {
            return Err(Error::PermissionDenied);
        };

        let item_status_dict = ItemStatusData::instance();
        item_status_dict.set(&item_id, value);

        Ok(())
    }

    fn get_item_by_index(&self, owner: Key, index: U256) -> Option<MarketItemId> {
        OwnedTokens::instance().get_item_by_index(&owner, &index)
    }

    fn validate_item_ids(&self, item_ids: Vec<MarketItemId>) -> bool {
        for item_id in &item_ids {
            if self.owner_of(*item_id).is_some() {
                return false;
            }
        }
        true
    }

    fn create_market_item(
        &mut self,
        recipient: Key,
        item_ids: Vec<MarketItemId>,
        nft_contract_addresses: Vec<NFTContractAddress>,
        item_asking_prices: Vec<U512>,
        item_token_ids: Vec<U256>,
    ) -> Result<Vec<MarketItemId>, Error> {
        if item_ids.len() != nft_contract_addresses.len() {
            return Err(Error::WrongArguments);
        };
        if item_ids.len() != item_asking_prices.len() {
            return Err(Error::WrongArguments);
        };
        if item_ids.len() != item_token_ids.len() {
            return Err(Error::WrongArguments);
        };

        for item_id in &item_ids {
            if self.owner_of(*item_id).is_some() {
                return Err(Error::MarketItemIdAlreadyExists);
            }
        }
        // TODO check is owner

        // TODO check this contract approved

        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let nft_market_item_ids_dict = NFTMarketItemIds::instance();
        let nft_contract_addresses_dict = NFTContractAddresses::instance();
        let item_asking_prices_dict = ItemAskingPriceData::instance();
        let item_token_ids_dict = ItemTokenIdData::instance();
        let item_status_dict = ItemStatusData::instance();

        let zipped = zip!(&item_ids, &nft_contract_addresses, &item_token_ids);
        for (item_id, (nft_contract_address, item_token_id)) in zipped {
            nft_contract_addresses_dict.set(item_id, nft_contract_address.clone());
            owners_dict.set(item_id, recipient);
            owned_tokens_dict.set_token(&recipient, item_id);
            item_status_dict.set(item_id, String::from(ITEM_STATUS_AVAILABLE));
            item_token_ids_dict.set(item_id, *item_token_id);
            nft_market_item_ids_dict.set(item_token_id, *item_id);
        }

        for (item_id, item_asking_price) in item_ids.iter().zip(item_asking_prices) {
            item_asking_prices_dict.set(item_id, item_asking_price);
        }

        let created_items_count: U256 = From::<u64>::from(item_ids.len().try_into().unwrap());
        let new_total_supply = data::total_supply()
            .checked_add(created_items_count)
            .unwrap();
        data::set_total_supply(new_total_supply);

        self.emit(MarketEvent::CreateItem {
            recipient,
            item_ids: item_ids.clone(),
        });
        Ok(item_ids)
    }

    fn process_market_sale(
        &mut self,
        recipient: Key,
        item_id: MarketItemId,
        market_offer_purse: URef,
    ) -> Result<(), Error> {
        // Check item status available
        if self.item_status(item_id).unwrap_or_revert() != *ITEM_STATUS_AVAILABLE {
            return Err(Error::MarketItemNotAvailable);
        };

        // Manage payment
        let asking_price = self.item_asking_price(item_id).unwrap_or_revert();
        let mint = system::get_mint();

        let balance: Option<U512> = runtime::call_contract(
            mint,
            METHOD_BALANCE,
            runtime_args! {
                ARG_PURSE => market_offer_purse,
            },
        );
        match balance {
            None => runtime::revert(ApiError::User(Error::BalanceNotFound as u16)),
            Some(balance) if balance == asking_price => (),
            _ => runtime::revert(ApiError::User(Error::BalanceMismatch as u16)),
        }

        let value_ref = storage::new_uref(balance);
        runtime::put_key("balance", Key::URef(value_ref));

        let value_ref = storage::new_uref(asking_price);
        runtime::put_key("asking_price", Key::URef(value_ref));

        let nft_contract_hash = self.item_nft_contract_address(item_id).unwrap();
        let token_id = self.item_token_id(item_id).unwrap();
        let owner = self.owner_of(item_id).unwrap_or_revert();

        let _: () = runtime::call_contract(
            nft_contract_hash,
            "transfer_from",
            runtime_args! {
                "sender" => owner,
                "recipient" => recipient,
                "token_ids" => vec![token_id]
            },
        );
        // TODO check ownership transferred to buyer

        // transfer money to seller/owner
        system::transfer_from_purse_to_account(
            market_offer_purse,
            owner.into_account().unwrap_or_revert(),
            asking_price,
            None,
        )
            .unwrap_or_revert();

        self.set_item_status(item_id, ITEM_STATUS_SOLD.to_string())
            .unwrap_or_revert();
        self.emit(MarketEvent::SoldItem { recipient, item_id });
        Ok(())
    }

    fn emit(&mut self, event: MarketEvent) {
        data::emit(&event);
    }
}
