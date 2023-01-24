use super::*;

#[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Bid {
    pub bidder: AccountId,
    pub price: Balance,
    pub amount: u64
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum BiddingError {
    InvalidAuctionId,
    InvalidBidId,
    AuctionIsNotActiveNow,
    CallerIsNotOriginalBidder,
    BidBelowStartingPrice,
    ZeroOrTooMany,
    FractionalUnitPrice
}

impl Bid {
    pub fn make(auction: &Auction, bidder: AccountId, price: Balance, amount: u64, current_time: Timestamp) -> Result<Self, BiddingError> {
        // verify operation corectness
        if !auction.is_active(current_time) {
            return Err(BiddingError::AuctionIsNotActiveNow);
        }
        if !(1..=auction.amount).contains(&amount) {
            return Err(BiddingError::ZeroOrTooMany);
        }
        if price % amount as u128 != 0 {
            return Err(BiddingError::FractionalUnitPrice);
        }
        if price / (amount as u128) < auction.starting_price {
            return Err(BiddingError::BidBelowStartingPrice);
        }

        Ok(Bid {bidder, price, amount})
    }
    
    pub fn increase(&mut self, auction: &Auction, extra_price: Balance, caller: AccountId, current_time: Timestamp) -> Result<(), BiddingError> {
        // verify operation corectness
        if !auction.is_active(current_time) {
            return Err(BiddingError::AuctionIsNotActiveNow);
        }
        if caller != self.bidder {
            return Err(BiddingError::CallerIsNotOriginalBidder);
        }
        if extra_price % self.amount as u128 != 0 {
            return Err(BiddingError::FractionalUnitPrice);
        }
        
        // increase the offer
        self.price += extra_price;
        Ok(())
    }
}

impl ChainOfBids {
    
}