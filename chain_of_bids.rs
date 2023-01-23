#![cfg_attr(not(feature = "std"), no_std)]
#![feature(proc_macro_hygiene)]
#![feature(result_contains_err)]

use ink_lang as ink;

#[ink::contract]
mod chain_of_bids {
    use ink_prelude::string::String;
    use ink_storage::{traits::SpreadAllocate, Mapping};

    mod auction;
    use auction::{Auction, AuctionCreationError, AuctionFinalizationError, bidding::{Bid, BiddingError}};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ChainOfBids {
        fee_denominator: u64, 
        fee_balance: Balance,
        owner: AccountId,

        number_of_auctions: u64,
        auctions: Mapping<u64, Auction>, // auction_id -> Auction
        bids: Mapping<(u64, u64), Bid>   // (auction_id, bid_id) -> Auction
    }

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum QueryError {
        InvalidAuctionId,
        InvalidAuctionOrBidId
    }
    
    // messages
    impl ChainOfBids {
        // contract management / getters
        #[ink(constructor)]
        pub fn new(fee_denominator: u64) -> Self {
            if fee_denominator == 0 {
                panic!("Invalid denominator");
            }

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.fee_denominator = fee_denominator;
                contract.owner = Self::env().caller();
                contract.number_of_auctions = 0;
            })
        }
        
        #[ink(message)]
        pub fn get_fee_denominator(&self) -> u64 {
            self.fee_denominator
        }
        
        #[ink(message)]
        pub fn collect_fees(&mut self) -> Result<(),()> {
            let caller = Self::env().caller();
            if caller != self.owner {
                return Err(())
            }
            
            if Self::env().transfer(caller, self.fee_balance).is_err() { panic!(); }
            self.fee_balance = 0;
            Ok(())
        }

        // auction list management / getters
        #[ink(message)]
        pub fn get_number_of_auctions(&self) -> u64 {
            self.number_of_auctions
        }

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
        
        
        // auction management / getters
        #[ink(message)]
        pub fn get_auction_name(&self, auction_id: u64) -> Result<String, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.name)
        }

        #[ink(message)]
        pub fn get_auction_description(&self, auction_id: u64) -> Result<String, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.description)
        }

        #[ink(message)]
        pub fn get_auction_owner(&self, auction_id: u64) -> Result<AccountId, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.owner)
        }

        #[ink(message)]
        pub fn get_auction_finalization_status(&self, auction_id: u64) -> Result<bool, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.finalized)
        }

        #[ink(message)]
        pub fn get_auction_end_period_start(&self, auction_id: u64) -> Result<Timestamp, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.end_period_start)
        }

        #[ink(message)]
        pub fn get_auction_end_period_end(&self, auction_id: u64) -> Result<Timestamp, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.end_period_stop)
        }

        #[ink(message)]
        pub fn get_auction_number_of_bids(&self, auction_id: u64) -> Result<u64, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.number_of_bids)
        }

        #[ink(message)]
        pub fn finalize_auction(&mut self, auction_id: u64) -> Result<(), AuctionFinalizationError> {
            // get auction
            let mut auction = self.auctions.get(auction_id).ok_or(AuctionFinalizationError::InvalidAuctionId)?;
            let current_time = Self::env().block_timestamp();
            
            // check whether finalization is possible
            if auction.finalized {
                return Err(AuctionFinalizationError::AuctionIsAlreadyFinalized);
            }
            if current_time < auction.end_period_start {
                return Err(AuctionFinalizationError::TooEarlyToFinish);
            }
            if current_time <= auction.end_period_stop && Self::env().caller() != auction.owner {
                return Err(AuctionFinalizationError::CallerIsNotOwner);
            }

            // mark as finalized
            auction.finalized = true;
            self.auctions.insert(auction_id, &auction);
            
            // if no bids were made, there's nothing else to do
            if auction.number_of_bids == 0 {
                return Ok(())
            }
            
            // find highest bid
            let mut highest_bid_id = 0;
            let mut highest_bid = self.bids.get((auction_id, 0)).unwrap();
            for i in 1..auction.number_of_bids {
                let bid = self.bids.get((auction_id, i)).unwrap();
                if bid.price > highest_bid.price {
                    highest_bid_id = i;
                    highest_bid = bid;
                }
            }
            
            // transfer the money back to the loosers
            for i in 0..auction.number_of_bids {
                if i != highest_bid_id {
                    let bid = self.bids.get((auction_id, i)).unwrap();
                    if Self::env().transfer(bid.bidder, bid.price).is_err() { panic!(); }
                }
            }
            
            // transfer the winner's money to auction owner
            if Self::env().transfer(auction.owner, highest_bid.price).is_err() { panic!(); }

            Ok(())
        }
        

        // bids
        #[ink(message)]
        pub fn get_bid_price(&self, auction_id: u64, bid_id: u64) -> Result<Balance, QueryError> {
            let bid = self.bids.get((auction_id, bid_id)).ok_or(QueryError::InvalidAuctionOrBidId)?;
            Ok(bid.price)
        }

        #[ink(message)]
        pub fn get_bid_bidder(&self, auction_id: u64, bid_id: u64) -> Result<AccountId, QueryError> {
            let bid = self.bids.get((auction_id, bid_id)).ok_or(QueryError::InvalidAuctionOrBidId)?;
            Ok(bid.bidder)
        }
        
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
        
        #[ink(message, payable)]
        pub fn increase_the_bid(&mut self, auction_id: u64, bid_id: u64) -> Result<(), BiddingError> {
            // find proper auction and bid
            let mut auction = self.auctions.get(auction_id).ok_or(BiddingError::InvalidAuctionId)?;
            let mut bid = self.bids.get((auction_id, bid_id)).ok_or(BiddingError::InvalidBidId)?;
            
            // check whether operation is possible
            if auction.is_finished(Self::env().block_timestamp()) {
                return Err(BiddingError::AuctionIsAlreadyFinished);
            }
            if bid.bidder != Self::env().caller() {
                return Err(BiddingError::CallerIsNotOriginalBidder);
            }
            
            // increase the bid
            let additional_price = Self::env().transferred_value();
            bid.price += additional_price;
            self.bids.insert((auction_id, bid_id), &bid);
                
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests;