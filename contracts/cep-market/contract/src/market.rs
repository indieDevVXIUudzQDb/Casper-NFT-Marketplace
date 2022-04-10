use crate::data::{
    Allowances, ItemAskingPriceData, ItemStatusData, ItemTokenIdData, NFTContractAddresses,
    OwnedTokens, Owners,
};
use crate::{
    data::{self},
    event::MarketEvent,
    Meta, NFTContractAddress, TokenId, ITEM_STATUS_AVAILABLE, ITEM_STATUS_SOLD,
};
use alloc::{string::String, vec::Vec};
use casper_types::{ApiError, Key, U256};
use contract_utils::{ContractContext, ContractStorage};
use core::convert::TryInto;

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
    fn init(&mut self, name: String, symbol: String, meta: Meta) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_meta(meta);
        data::set_total_supply(U256::zero());
        Owners::init();
        OwnedTokens::init();
        NFTContractAddresses::init();
        ItemAskingPriceData::init();
        ItemStatusData::init();
        ItemTokenIdData::init();
        Allowances::init();
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

    fn owner_of(&self, item_id: TokenId) -> Option<Key> {
        Owners::instance().get(&item_id)
    }

    fn item_nft_contract_address(&self, item_id: TokenId) -> Option<NFTContractAddress> {
        NFTContractAddresses::instance().get(&item_id)
    }

    fn set_item_nft_contract_address(
        &mut self,
        item_id: TokenId,
        nft_contract_address: NFTContractAddress,
    ) -> Result<(), Error> {
        if self.owner_of(item_id).is_none() {
            return Err(Error::TokenIdDoesntExist);
        };

        let nft_contract_addresses_dict = NFTContractAddresses::instance();
        nft_contract_addresses_dict.set(&item_id, nft_contract_address);

        Ok(())
    }

    fn item_asking_price(&self, item_id: TokenId) -> Option<U256> {
        ItemAskingPriceData::instance().get(&item_id)
    }

    fn item_token_id(&self, item_id: TokenId) -> Option<U256> {
        ItemTokenIdData::instance().get(&item_id)
    }

    fn item_status(&self, item_id: TokenId) -> Option<String> {
        ItemStatusData::instance().get(&item_id)
    }

    // CLType::List(Box::new(MarketItemInstance::cl_type()))
    // fn available_items(&self) -> MarketItemList {
    //     // let length = Items::instance().get_length();
    //     // let mut market_items = Vec::new();
    //     let mut market_items = MarketItemList{
    //         ids: Vec::new()
    //     };
    //     let mut index = U256::zero();
    //     loop {
    //         let item = Items::instance().get_item(&index);
    //         if item != None {
    //            // let market_item = market_item::init(
    //            //     self.item_nft_contract_address(index).unwrap(),
    //            //      self.item_asking_price(index).unwrap(),
    //            //     self.item_token_id(index).unwrap(),
    //            //     self.item_status(index).unwrap(),
    //            // );
    //             // market_items.push(market_item);
    //             market_items.ids.push(self.item_token_id(index).unwrap());
    //         } else {
    //             break;
    //         }
    //         index += U256::from("1");
    //     }
    //     market_items
    // }

    fn set_item_asking_price(
        &mut self,
        item_id: TokenId,
        item_asking_price: U256,
    ) -> Result<(), Error> {
        if self.owner_of(item_id).is_none() {
            return Err(Error::TokenIdDoesntExist);
        };

        let item_asking_price_dict = ItemAskingPriceData::instance();
        item_asking_price_dict.set(&item_id, item_asking_price);

        Ok(())
    }

    fn set_item_status(&mut self, item_id: TokenId, value: String) -> Result<(), Error> {
        if self.owner_of(item_id).is_none() {
            return Err(Error::TokenIdDoesntExist);
        };

        let item_status_dict = ItemStatusData::instance();
        item_status_dict.set(&item_id, value);

        Ok(())
    }

    fn get_item_by_index(&self, owner: Key, index: U256) -> Option<TokenId> {
        OwnedTokens::instance().get_item_by_index(&owner, &index)
    }

    fn validate_item_ids(&self, item_ids: Vec<TokenId>) -> bool {
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
        item_ids: Vec<TokenId>,
        nft_contract_addresses: Vec<NFTContractAddress>,
        item_asking_prices: Vec<U256>,
        item_token_ids: Vec<U256>,
    ) -> Result<Vec<TokenId>, Error> {
        if item_ids.len() != nft_contract_addresses.len() {
            return Err(Error::WrongArguments);
        };

        for item_id in &item_ids {
            if self.owner_of(*item_id).is_some() {
                return Err(Error::TokenIdAlreadyExists);
            }
        }

        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let nft_contract_addresses_dict = NFTContractAddresses::instance();
        let item_asking_prices_dict = ItemAskingPriceData::instance();
        let item_token_ids_dict = ItemTokenIdData::instance();
        let item_status_dict = ItemStatusData::instance();

        for (item_id, meta) in item_ids.iter().zip(&nft_contract_addresses) {
            nft_contract_addresses_dict.set(item_id, meta.clone());
            owners_dict.set(item_id, recipient);
            owned_tokens_dict.set_token(&recipient, item_id);
            // items_dict.set(item_id,"hello world".to_string());
        }

        for (item_id, item_asking_price) in item_ids.iter().zip(&item_asking_prices) {
            item_asking_prices_dict.set(item_id, *item_asking_price);
            item_status_dict.set(item_id, String::from(ITEM_STATUS_AVAILABLE));
        }

        for (item_id, item_token_id) in item_ids.iter().zip(&item_token_ids) {
            item_token_ids_dict.set(item_id, *item_token_id);
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

    fn process_market_sale(&mut self, recipient: Key, item_id: TokenId) -> Result<TokenId, Error> {
        if self.owner_of(item_id).is_none() {
            return Err(Error::TokenIdDoesntExist);
        }
        let result = self.set_item_status(item_id, String::from(ITEM_STATUS_SOLD));
        if result.is_err() {
            return Err(Error::TokenIdDoesntExist);
        }
        self.emit(MarketEvent::SoldItem { recipient, item_id });
        Ok(item_id)
    }

    fn emit(&mut self, event: MarketEvent) {
        data::emit(&event);
    }
}
