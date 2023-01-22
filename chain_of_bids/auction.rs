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
    pub finished: bool,
    pub end_period_start: Timestamp, // before that moment, the auction cannot be finished
    pub end_period_stop: Timestamp // after that moment, the auction is concidered finished (no matter what)
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AuctionCreationError {
    InvalidNameLength,
    InvalidDescriptionLength,
    InvalidEndPeriod
}

impl Auction {
    // this constructor should be used instead of raw { }
    pub fn new(
        name: String,
        description: String,
        owner: AccountId,
        end_period_start: Timestamp,
        end_period_stop: Timestamp
    ) -> Result<Self, AuctionCreationError> {
        // verify input correctness
        if !(1..65).contains(&name.len()) {
            return Err(AuctionCreationError::InvalidNameLength);
        } 
        if !(0..4097).contains(&description.len()) {
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

            finished: false,
            end_period_start,
            end_period_stop
        })
    }

    pub fn is_finished(&mut self, current_time: Timestamp) -> bool {
        // if already finished, just return that
        if self.finished {
            return true;
        }
        
        // if absolute finishing time has passed, set to finished
        if self.end_period_stop < current_time {
            self.finished = true;
            return true;
        }
        
        // otherwise, the auction is still active
        return false;
    }
}