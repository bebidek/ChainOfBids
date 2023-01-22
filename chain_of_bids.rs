#![cfg_attr(not(feature = "std"), no_std)]
#![feature(proc_macro_hygiene)]
#![feature(result_contains_err)]

use ink_lang as ink;

#[ink::contract]
mod chain_of_bids {
    use ink_prelude::string::String;
    use ink_storage::{traits::SpreadAllocate, Mapping};

    mod auction;
    use auction::{Auction, AuctionCreationError};

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
        pub fn create_new_auction(
            &mut self,
            name: String,
            description: String,
            end_period_start: Timestamp,
            end_period_stop: Timestamp
        ) -> Result<u64, AuctionCreationError> {
            // make action entry (with validation)
            let auction = Auction::new(name, description, Self::env().caller(), end_period_start, end_period_stop)?;
            let auction_id = self.number_of_auctions;

            // add new entry to the storage
            self.auctions.insert(auction_id, &auction);
            self.number_of_auctions += 1;
            Ok(auction_id)
        }
        
        #[ink(message)]
        pub fn get_number_of_auctions(&self) -> u64 {
            self.number_of_auctions
        }
    }
}

#[cfg(test)]
mod tests;