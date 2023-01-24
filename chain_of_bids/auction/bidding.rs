use super::*;

#[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Bid {
    pub bidder: AccountId,
    pub price: Balance
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum BiddingError {
    InvalidAuctionId,
    InvalidBidId,
    AuctionIsNotActiveNow,
    CallerIsNotOriginalBidder,
    BidBelowStartingPrice
}

impl Bid {
    pub fn make(auction: &Auction, bidder: AccountId, price: Balance, current_time: Timestamp) -> Result<Self, BiddingError> {
        // verify operation corectness
        if !auction.is_active(current_time) {
            return Err(BiddingError::AuctionIsNotActiveNow);
        }
        if price < auction.starting_price {
            return Err(BiddingError::BidBelowStartingPrice);
        }

        Ok(Bid {bidder, price})
    }
    
    pub fn increase(&mut self, auction: &Auction, extra_price: Balance, caller: AccountId, current_time: Timestamp) -> Result<(), BiddingError> {
        // verify operation corectness
        if !auction.is_active(current_time) {
            return Err(BiddingError::AuctionIsNotActiveNow);
        }
        if caller != self.bidder {
            return Err(BiddingError::CallerIsNotOriginalBidder);
        }
        
        // increase the offer
        self.price += extra_price;
        Ok(())
    }
}

impl ChainOfBids {
    
}