use crate::{data::{self}, event::MarketEvent, Meta, NFTContractAddress, TokenId};
use alloc::{string::String, vec::Vec};
use core::convert::TryInto;
use casper_types::{ApiError, Key, U256};
use contract_utils::{ContractContext, ContractStorage};
use crate::data::{Allowances, NFTContractAddresses, OwnedTokens, Owners};

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

    fn set_item_nft_contract_address(&mut self, item_id: TokenId, nft_contract_address: NFTContractAddress) -> Result<(), Error> {
        if self.owner_of(item_id).is_none() {
            return Err(Error::TokenIdDoesntExist);
        };

        let nft_contract_addresses_dict = NFTContractAddresses::instance();
        nft_contract_addresses_dict.set(&item_id, nft_contract_address);

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

    fn mint(
        &mut self,
        recipient: Key,
        item_ids: Vec<TokenId>,
        nft_contract_addresses: Vec<NFTContractAddress>,
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

        for (item_id, meta) in item_ids.iter().zip(&nft_contract_addresses) {
            nft_contract_addresses_dict.set(item_id, meta.clone());
            owners_dict.set(item_id, recipient);
            owned_tokens_dict.set_token(&recipient, item_id);
        }

        let minted_tokens_count: U256 = From::<u64>::from(item_ids.len().try_into().unwrap());
        let new_total_supply = data::total_supply()
            .checked_add(minted_tokens_count)
            .unwrap();
        data::set_total_supply(new_total_supply);

        self.emit(MarketEvent::Mint {
            recipient,
            item_ids: item_ids.clone(),
        });
        Ok(item_ids)
    }

    fn mint_copies(
        &mut self,
        recipient: Key,
        item_ids: Vec<TokenId>,
        item_nft_contract_address: NFTContractAddress,
        count: u32,
    ) -> Result<Vec<TokenId>, Error> {
        let item_nft_contract_address = vec![item_nft_contract_address; count.try_into().unwrap()];
        self.mint(recipient, item_ids, item_nft_contract_address)
    }

    fn burn(&mut self, owner: Key, item_ids: Vec<TokenId>) -> Result<(), Error> {
        let spender = self.get_caller();
        if spender != owner {
            for item_id in &item_ids {
                if !self.is_approved(owner, *item_id, spender) {
                    return Err(Error::PermissionDenied);
                }
            }
        }
        self.burn_internal(owner, item_ids)
    }

    fn burn_internal(&mut self, owner: Key, item_ids: Vec<TokenId>) -> Result<(), Error> {
        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let nft_contract_addresses_dict = NFTContractAddresses::instance();
        let allowances_dict = Allowances::instance();

        for item_id in &item_ids {
            match owners_dict.get(item_id) {
                Some(owner_of_key) => {
                    if owner_of_key != owner {
                        return Err(Error::PermissionDenied);
                    }
                }
                None => {
                    return Err(Error::TokenIdDoesntExist);
                }
            }
        }

        for item_id in &item_ids {
            owned_tokens_dict.remove_token(&owner, item_id);
            nft_contract_addresses_dict.remove(item_id);
            owners_dict.remove(item_id);
            allowances_dict.remove(&owner, item_id);
        }

        let burnt_tokens_count: U256 = From::<u64>::from(item_ids.len().try_into().unwrap());
        let new_total_supply = data::total_supply()
            .checked_sub(burnt_tokens_count)
            .unwrap();
        data::set_total_supply(new_total_supply);

        self.emit(MarketEvent::Burn { owner, item_ids });
        Ok(())
    }

    fn approve(&mut self, spender: Key, item_ids: Vec<TokenId>) -> Result<(), Error> {
        let caller = self.get_caller();
        for item_id in &item_ids {
            match self.owner_of(*item_id) {
                None => return Err(Error::WrongArguments),
                Some(owner) if owner != caller => return Err(Error::PermissionDenied),
                Some(_) => Allowances::instance().set(&caller, item_id, spender),
            }
        }
        self.emit(MarketEvent::Approve {
            owner: caller,
            spender,
            item_ids,
        });
        Ok(())
    }

    fn get_approved(&self, owner: Key, item_id: TokenId) -> Option<Key> {
        Allowances::instance().get(&owner, &item_id)
    }

    fn transfer(&mut self, recipient: Key, item_ids: Vec<TokenId>) -> Result<(), Error> {
        self.transfer_from(self.get_caller(), recipient, item_ids)
    }

    fn transfer_from(
        &mut self,
        owner: Key,
        recipient: Key,
        item_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        let spender = self.get_caller();

        if owner != spender {
            let allowances_dict = Allowances::instance();
            for item_id in &item_ids {
                if !self.is_approved(owner, *item_id, spender) {
                    return Err(Error::PermissionDenied);
                }
                allowances_dict.remove(&owner, item_id);
            }
        }
        self.transfer_from_internal(owner, recipient, item_ids)
    }

    fn transfer_from_internal(
        &mut self,
        owner: Key,
        recipient: Key,
        item_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();

        for item_id in &item_ids {
            match owners_dict.get(item_id) {
                Some(owner_of_key) => {
                    if owner_of_key != owner {
                        return Err(Error::PermissionDenied);
                    }
                }
                None => {
                    return Err(Error::TokenIdDoesntExist);
                }
            }
        }

        for item_id in &item_ids {
            owned_tokens_dict.remove_token(&owner, item_id);
            owned_tokens_dict.set_token(&recipient, item_id);
            owners_dict.set(item_id, recipient);
        }

        self.emit(MarketEvent::Transfer {
            sender: owner,
            recipient,
            item_ids,
        });
        Ok(())
    }

    fn is_approved(&self, owner: Key, item_id: TokenId, spender: Key) -> bool {
        let allowances_dict = Allowances::instance();
        if let Some(spender_of) = allowances_dict.get(&owner, &item_id) {
            if spender_of == spender {
                return true;
            }
        }
        false
    }

    fn emit(&mut self, event: MarketEvent) {
        data::emit(&event);
    }
}
