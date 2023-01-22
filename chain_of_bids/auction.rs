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

impl Auction {
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