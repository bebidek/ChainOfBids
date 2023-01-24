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
        pub fn get_fee_balance(&self) -> Balance {
            self.fee_balance
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
            amount: u64,
            start_time: Timestamp,
            end_period_start: Timestamp,
            end_period_stop: Timestamp,
            starting_price: Balance
        ) -> Result<u64, AuctionCreationError> {
            // delegate
            let auction = Auction::new(name, description, Self::env().caller(), amount, start_time, end_period_start, end_period_stop, starting_price)?;
            let auction_id = self.number_of_auctions;

            // insert into structures
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
        pub fn get_auction_amount(&self, auction_id: u64) -> Result<u64, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.amount)
        }

        #[ink(message)]
        pub fn get_auction_finalization_status(&self, auction_id: u64) -> Result<bool, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.finalized)
        }

        #[ink(message)]
        pub fn get_auction_start_time(&self, auction_id: u64) -> Result<Timestamp, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.start_time)
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
        pub fn get_auction_starting_price(&self, auction_id: u64) -> Result<Balance, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.starting_price)
        }

        #[ink(message)]
        pub fn get_auction_number_of_bids(&self, auction_id: u64) -> Result<u64, QueryError> {
            let auction = self.auctions.get(auction_id).ok_or(QueryError::InvalidAuctionId)?;
            Ok(auction.number_of_bids)
        }
        
        #[ink(message)]
        pub fn finalize_auction(&mut self, auction_id: u64, forefront: ink_prelude::vec::Vec<u64>) -> Result<(), AuctionFinalizationError> {
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
            
            // greater means here: more important (higher unit price or earlier)
            fn bid_less_than(bid1: &Bid, bid1_i: u64, bid2: &Bid, bid2_i: u64) -> bool {
                if bid1.price / (bid1.amount as u128) != bid2.price / (bid2.amount as u128) {
                    return bid1.price / (bid1.amount as u128) < bid2.price / (bid2.amount as u128);
                } else {
                    return bid1_i > bid2_i;
                }
            }
            
            // perform money transfers for forefront
            // also, verify forefront list: check whether they are sorted in strictly decreasing order
            let mut items_left = auction.amount;
            let mut last_bid: Option<Bid> = None;
            
            for i in 0..forefront.len() {
                // get this bid
                let bid_id = forefront[i];
                if !(0..auction.number_of_bids).contains(&bid_id) {
                    return Err(AuctionFinalizationError::InvalidForefrontVector);
                }
                let this_bid = self.bids.get((auction_id, bid_id)).ok_or(AuctionFinalizationError::InvalidForefrontVector)?;
                
                // verify order
                if last_bid.is_some() && !bid_less_than(&this_bid, bid_id, &last_bid.unwrap(), forefront[i - 1]) {
                    return Err(AuctionFinalizationError::InvalidForefrontVector);
                }
                
                // transfer the money
                if this_bid.amount <= items_left { // it's a winner
                    items_left -= this_bid.amount;
                    let fee = this_bid.price / self.fee_denominator as u128;
                    if Self::env().transfer(auction.owner, this_bid.price - fee).is_err() { panic!(); }
                    self.fee_balance += fee;
                } else { // it's a looser
                    if Self::env().transfer(this_bid.bidder, this_bid.price).is_err() { panic!(); }
                }
                
                // save for next iterations
                last_bid = Some(this_bid);
            }
            
            // verify that all other (non-forefront) are smaller and that they lost (and give them tyey money back)
            let mut greater_or_equal_left = forefront.len();
            
            for i in 0..auction.number_of_bids {
                let this_bid = self.bids.get((auction_id, i)).ok_or(AuctionFinalizationError::DummyError)?;
                if last_bid.is_some() && !bid_less_than(&this_bid, i, last_bid.as_ref().unwrap(), forefront[forefront.len() - 1]) {
                    if greater_or_equal_left == 0 {
                        return Err(AuctionFinalizationError::InvalidForefrontVector);
                    }
                    greater_or_equal_left -= 1;
                } else if this_bid.amount <= items_left {
                    return Err(AuctionFinalizationError::InvalidForefrontVector);
                } else {
                    if Self::env().transfer(this_bid.bidder, this_bid.price).is_err() { panic!(); }
                }
            }
            
            // update auction structure
            auction.finalized = true;
            self.auctions.insert(auction_id, &auction);

            Ok(())
        }

        // bids
        #[ink(message)]
        pub fn get_bid_bidder(&self, auction_id: u64, bid_id: u64) -> Result<AccountId, QueryError> {
            let bid = self.bids.get((auction_id, bid_id)).ok_or(QueryError::InvalidAuctionOrBidId)?;
            Ok(bid.bidder)
        }

        #[ink(message)]
        pub fn get_bid_price(&self, auction_id: u64, bid_id: u64) -> Result<Balance, QueryError> {
            let bid = self.bids.get((auction_id, bid_id)).ok_or(QueryError::InvalidAuctionOrBidId)?;
            Ok(bid.price)
        }

        #[ink(message)]
        pub fn get_bid_amount(&self, auction_id: u64, bid_id: u64) -> Result<u64, QueryError> {
            let bid = self.bids.get((auction_id, bid_id)).ok_or(QueryError::InvalidAuctionOrBidId)?;
            Ok(bid.amount)
        }
        
        #[ink(message, payable)]
        pub fn make_a_bid(&mut self, auction_id: u64, amount: u64) -> Result<u64, BiddingError> {
            // find proper auction
            let mut auction = self.auctions.get(auction_id).ok_or(BiddingError::InvalidAuctionId)?;

            // delegate
            let bid = Bid::make(&mut auction, Self::env().caller(), Self::env().transferred_value(), amount, Self::env().block_timestamp())?;
            let bid_id = auction.number_of_bids;

            // insert into structures
            auction.number_of_bids += 1;
            self.auctions.insert(auction_id, &auction);
            self.bids.insert((auction_id, bid_id), &bid);

            Ok(bid_id)
        }
        
        #[ink(message, payable)]
        pub fn increase_the_bid(&mut self, auction_id: u64, bid_id: u64) -> Result<(), BiddingError> {
            // find proper auction and bid
            let auction = self.auctions.get(auction_id).ok_or(BiddingError::InvalidAuctionId)?;
            let mut bid = self.bids.get((auction_id, bid_id)).ok_or(BiddingError::InvalidBidId)?;
            
            // delegate and insert
            bid.increase(&auction, Self::env().transferred_value(), Self::env().caller(), Self::env().block_timestamp())?;
            self.bids.insert((auction_id, bid_id), &bid);
                
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests;