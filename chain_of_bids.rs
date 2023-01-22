#![cfg_attr(not(feature = "std"), no_std)]
#![feature(proc_macro_hygiene)]
#![feature(result_contains_err)]

use ink_lang as ink;

#[ink::contract]
mod chain_of_bids {
    use ink_prelude::string::String;
    use ink_storage::{traits::SpreadAllocate, Mapping};

    mod auction;
    use auction::{Auction, AuctionCreationError, bidding::{Bid, BiddingError}};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ChainOfBids {
        number_of_auctions: u64,
        auctions: Mapping<u64, Auction>, // auction_id -> Auction
        bids: Mapping<(u64, u64), Bid>   // (auction_id, bid_id) -> Auction
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
        
        // bidding
        #[ink(message, payable)]
        pub fn make_a_bid(&mut self, auction_id: u64) -> Result<u64, BiddingError> {
            // find proper auction
            let mut auction = self.auctions.get(auction_id).ok_or(BiddingError::InvalidAuctionId)?;

            // create (and validate) the new bid
            let caller = Self::env().caller();
            let price = Self::env().transferred_value();
            let current_time = Self::env().block_timestamp();
            let bid = Bid::make(&mut auction, caller, price, current_time)?;

            // add to structures
            let bid_id = auction.number_of_bids;
            auction.number_of_bids += 1;
            self.auctions.insert(auction_id, &auction);
            self.bids.insert((auction_id, bid_id), &bid);

            Ok(bid_id)
        }
    }
}

#[cfg(test)]
mod tests;