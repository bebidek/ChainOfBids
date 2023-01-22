use super::*;

#[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Bid {
    bidder: AccountId,
    price: Balance
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum BiddingError {
    InvalidAuctionId,
    AuctionIsAlreadyFinished
}

impl Bid {
    pub fn make(auction: &mut Auction, bidder: AccountId, price: Balance, current_time: Timestamp) -> Result<Self, BiddingError> {
        if auction.is_finished(current_time) {
            return Err(BiddingError::AuctionIsAlreadyFinished);
        }
        
        Ok(Bid {bidder, price})
    }
}
