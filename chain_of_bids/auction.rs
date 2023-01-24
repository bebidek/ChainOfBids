use super::*;
use ink_storage::{traits::{SpreadLayout, PackedLayout}};

#[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Auction {
    // main information
    pub name: String,
    pub description: String,
    pub owner: AccountId,

    // status / timing data
    pub finalized: bool, // finalized means that all funds have been transfered to bidders / owner
    pub start_time: Timestamp,       // before that moment, bidding is not possible
    pub end_period_start: Timestamp, // before that moment, the auction cannot be finished
    pub end_period_stop: Timestamp, // after that moment, the auction is concidered finished (no matter what)
    
    // bids
    pub starting_price: Balance,
    pub number_of_bids: u64
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AuctionCreationError {
    InvalidNameLength,
    InvalidDescriptionLength,
    InvalidEndPeriod
}


#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AuctionFinalizationError {
    InvalidAuctionId,
    AuctionIsAlreadyFinalized,
    TooEarlyToFinish,
    CallerIsNotOwner
}

impl Auction {
    // this constructor should be used instead of raw { }
    pub fn new(
        name: String,
        description: String,
        owner: AccountId,
        start_time: Timestamp,
        end_period_start: Timestamp,
        end_period_stop: Timestamp,
        starting_price: Balance
    ) -> Result<Self, AuctionCreationError> {
        // verify input correctness
        if !(1..=64).contains(&name.len()) {
            return Err(AuctionCreationError::InvalidNameLength);
        } 
        if !(0..=4096).contains(&description.len()) {
            return Err(AuctionCreationError::InvalidDescriptionLength);
        }
        if end_period_stop < end_period_start {
            return Err(AuctionCreationError::InvalidEndPeriod);
        }
        
        // create auction
        Ok(Auction {
            name,
            description,
            owner,

            finalized: false,
            start_time,
            end_period_start,
            end_period_stop,

            number_of_bids: 0,
            starting_price
        })
    }

    pub fn is_active(&self, current_time: Timestamp) -> bool {
        // active means that bidding is possible
        
        // if auction has not started yet
        if current_time < self.start_time {
            return false;
        }
        
        // if auction has already ended (because of timeout)
        if current_time > self.end_period_stop {
            return false;
        }
        
        // if auction is already finalized (including manually finished by owner)
        if self.finalized {
            return false;
        }
        
        return true;
    }
}

pub mod bidding;