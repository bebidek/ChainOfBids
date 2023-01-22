use ink_lang as ink;
use crate::chain_of_bids::ChainOfBids;

#[ink::test]
fn number_of_auctions_is_initially_zero() {
    let chain_of_bids = ChainOfBids::new();
    assert_eq!(chain_of_bids.get_number_of_auctions(), 0);
}

#[ink::test]
fn number_of_auction_increases() {
    let mut chain_of_bids = ChainOfBids::new();

    chain_of_bids.create_new_auction("name1".to_owned(), "description1".to_owned());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 1);

    chain_of_bids.create_new_auction("name2".to_owned(), "description2".to_owned());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 2);
}