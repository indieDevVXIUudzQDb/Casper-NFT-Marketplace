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

    fn owner_of(&self, token_id: TokenId) -> Option<Key> {
        Owners::instance().get(&token_id)
    }

    fn token_nft_contract_address(&self, token_id: TokenId) -> Option<NFTContractAddress> {
        NFTContractAddresses::instance().get(&token_id)
    }

    fn set_token_nft_contract_address(&mut self, token_id: TokenId, nft_contract_address: NFTContractAddress) -> Result<(), Error> {
        if self.owner_of(token_id).is_none() {
            return Err(Error::TokenIdDoesntExist);
        };

        let nft_contract_addresses_dict = NFTContractAddresses::instance();
        nft_contract_addresses_dict.set(&token_id, nft_contract_address);

        Ok(())
    }

    fn get_token_by_index(&self, owner: Key, index: U256) -> Option<TokenId> {
        OwnedTokens::instance().get_token_by_index(&owner, &index)
    }

    fn validate_token_ids(&self, token_ids: Vec<TokenId>) -> bool {
        for token_id in &token_ids {
            if self.owner_of(*token_id).is_some() {
                return false;
            }
        }
        true
    }

    fn mint(
        &mut self,
        recipient: Key,
        token_ids: Vec<TokenId>,
        nft_contract_addresses: Vec<NFTContractAddress>,
    ) -> Result<Vec<TokenId>, Error> {
        if token_ids.len() != nft_contract_addresses.len() {
            return Err(Error::WrongArguments);
        };

        for token_id in &token_ids {
            if self.owner_of(*token_id).is_some() {
                return Err(Error::TokenIdAlreadyExists);
            }
        }

        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let nft_contract_addresses_dict = NFTContractAddresses::instance();

        for (token_id, meta) in token_ids.iter().zip(&nft_contract_addresses) {
            nft_contract_addresses_dict.set(token_id, meta.clone());
            owners_dict.set(token_id, recipient);
            owned_tokens_dict.set_token(&recipient, token_id);
        }

        let minted_tokens_count: U256 = From::<u64>::from(token_ids.len().try_into().unwrap());
        let new_total_supply = data::total_supply()
            .checked_add(minted_tokens_count)
            .unwrap();
        data::set_total_supply(new_total_supply);

        self.emit(MarketEvent::Mint {
            recipient,
            token_ids: token_ids.clone(),
        });
        Ok(token_ids)
    }

    fn mint_copies(
        &mut self,
        recipient: Key,
        token_ids: Vec<TokenId>,
        token_nft_contract_address: NFTContractAddress,
        count: u32,
    ) -> Result<Vec<TokenId>, Error> {
        let token_nft_contract_address = vec![token_nft_contract_address; count.try_into().unwrap()];
        self.mint(recipient, token_ids, token_nft_contract_address)
    }

    fn burn(&mut self, owner: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let spender = self.get_caller();
        if spender != owner {
            for token_id in &token_ids {
                if !self.is_approved(owner, *token_id, spender) {
                    return Err(Error::PermissionDenied);
                }
            }
        }
        self.burn_internal(owner, token_ids)
    }

    fn burn_internal(&mut self, owner: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let nft_contract_addresses_dict = NFTContractAddresses::instance();
        let allowances_dict = Allowances::instance();

        for token_id in &token_ids {
            match owners_dict.get(token_id) {
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

        for token_id in &token_ids {
            owned_tokens_dict.remove_token(&owner, token_id);
            nft_contract_addresses_dict.remove(token_id);
            owners_dict.remove(token_id);
            allowances_dict.remove(&owner, token_id);
        }

        let burnt_tokens_count: U256 = From::<u64>::from(token_ids.len().try_into().unwrap());
        let new_total_supply = data::total_supply()
            .checked_sub(burnt_tokens_count)
            .unwrap();
        data::set_total_supply(new_total_supply);

        self.emit(MarketEvent::Burn { owner, token_ids });
        Ok(())
    }

    fn approve(&mut self, spender: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let caller = self.get_caller();
        for token_id in &token_ids {
            match self.owner_of(*token_id) {
                None => return Err(Error::WrongArguments),
                Some(owner) if owner != caller => return Err(Error::PermissionDenied),
                Some(_) => Allowances::instance().set(&caller, token_id, spender),
            }
        }
        self.emit(MarketEvent::Approve {
            owner: caller,
            spender,
            token_ids,
        });
        Ok(())
    }

    fn get_approved(&self, owner: Key, token_id: TokenId) -> Option<Key> {
        Allowances::instance().get(&owner, &token_id)
    }

    fn transfer(&mut self, recipient: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        self.transfer_from(self.get_caller(), recipient, token_ids)
    }

    fn transfer_from(
        &mut self,
        owner: Key,
        recipient: Key,
        token_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        let spender = self.get_caller();

        if owner != spender {
            let allowances_dict = Allowances::instance();
            for token_id in &token_ids {
                if !self.is_approved(owner, *token_id, spender) {
                    return Err(Error::PermissionDenied);
                }
                allowances_dict.remove(&owner, token_id);
            }
        }
        self.transfer_from_internal(owner, recipient, token_ids)
    }

    fn transfer_from_internal(
        &mut self,
        owner: Key,
        recipient: Key,
        token_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();

        for token_id in &token_ids {
            match owners_dict.get(token_id) {
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

        for token_id in &token_ids {
            owned_tokens_dict.remove_token(&owner, token_id);
            owned_tokens_dict.set_token(&recipient, token_id);
            owners_dict.set(token_id, recipient);
        }

        self.emit(MarketEvent::Transfer {
            sender: owner,
            recipient,
            token_ids,
        });
        Ok(())
    }

    fn is_approved(&self, owner: Key, token_id: TokenId, spender: Key) -> bool {
        let allowances_dict = Allowances::instance();
        if let Some(spender_of) = allowances_dict.get(&owner, &token_id) {
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
