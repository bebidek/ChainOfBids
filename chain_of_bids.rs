#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod chain_of_bids {
    use ink_prelude::string::String;
    use ink_storage::{traits::{SpreadAllocate, SpreadLayout, PackedLayout}, Mapping};

    #[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    struct Auction {
        name: String,
        description: String,
        owner: AccountId
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ChainOfBids {
        number_of_auctions: u64,
        auctions: Mapping<u64, Auction>
    }

    impl ChainOfBids {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.number_of_auctions = 0;
            })
        }
        
        // auction list management
        #[ink(message)]
        pub fn create_new_auction(&mut self, name: String, description: String) {
            let caller = Self::env().caller();
            let auction_id = self.number_of_auctions;
            
            let auction = Auction {
                name: name,
                description: description,
                owner: caller
            };
            
            self.auctions.insert(auction_id, &auction);
            self.number_of_auctions += 1;
        }
        
        #[ink(message)]
        pub fn get_number_of_auctions(&self) -> u64 {
            self.number_of_auctions
        }
    }
}

#[cfg(test)]
mod tests;