#![cfg_attr(not(feature = "std"), no_std)]
#![feature(proc_macro_hygiene)]
#![feature(result_contains_err)]

use ink_lang as ink;

#[ink::contract]
mod chain_of_bids {
    use ink_prelude::string::String;
    use ink_storage::{traits::SpreadAllocate, Mapping};

    mod auction;
    use auction::Auction;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ChainOfBids {
        number_of_auctions: u64,
        auctions: Mapping<u64, Auction>
    }

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum AuctionCreatingError {
        InvalidNameLength,
        InvalidDescriptionLength,
        InvalidEndPeriod
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
        pub fn create_new_auction(&mut self, name: String, description: String, end_period_start: Timestamp, end_period_stop: Timestamp) -> Result<u64, AuctionCreatingError> {
            let caller = Self::env().caller();
            
            // verify input correctness
            if !(1..65).contains(&name.len()) {
                return Err(AuctionCreatingError::InvalidNameLength);
            } 
            if !(0..4097).contains(&description.len()) {
                return Err(AuctionCreatingError::InvalidDescriptionLength);
            }
            if end_period_stop < end_period_start {
                return Err(AuctionCreatingError::InvalidEndPeriod);
            }
            
            // create auction entry
            let auction = Auction {
                name,
                description,
                owner: caller,

                finished: false,
                end_period_start,
                end_period_stop
            };
            let auction_id = self.number_of_auctions;
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