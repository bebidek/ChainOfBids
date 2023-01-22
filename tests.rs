
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

    assert!(chain_of_bids.create_new_auction("name1".to_owned(), "description1".to_owned(), 0, 0).is_ok());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 1);

    assert!(chain_of_bids.create_new_auction("name2".to_owned(), "description2".to_owned(), 0, 0).is_ok());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 2);
}

#[ink::test]
fn verifying_name_correctness_works() {
    let mut chain_of_bids = ChainOfBids::new();
    
    assert!(chain_of_bids.create_new_auction("".to_owned(), "description".to_owned(), 0, 0).is_err());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 0);

    assert!(chain_of_bids.create_new_auction("abcd".to_owned(), "description".to_owned(), 0, 0).is_ok());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 1);

    assert!(chain_of_bids.create_new_auction("x".repeat(64), "description".to_owned(), 0, 0).is_ok());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 2);

    assert!(chain_of_bids.create_new_auction("x".repeat(65), "description".to_owned(), 0, 0).is_err());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 2);
}

#[ink::test]
fn verifying_description_correctness_works() {
    let mut chain_of_bids = ChainOfBids::new();
    
    assert!(chain_of_bids.create_new_auction("abcd".to_owned(), "".to_owned(), 0, 0).is_ok());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 1);

    assert!(chain_of_bids.create_new_auction("abcd".to_owned(), "x".repeat(4096), 0, 0).is_ok());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 2);

    assert!(chain_of_bids.create_new_auction("abcd".to_owned(), "x".repeat(4097), 0, 0).is_err());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 2);
}

#[ink::test]
fn verifying_end_period_correctness_works() {
    let mut chain_of_bids = ChainOfBids::new();
    
    assert!(chain_of_bids.create_new_auction("abcd".to_owned(), "efgh".to_owned(), 42, 42).is_ok());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 1);

    assert!(chain_of_bids.create_new_auction("abcd".to_owned(), "efgh".to_owned(), 42, 84).is_ok());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 2);

    assert!(chain_of_bids.create_new_auction("abcd".to_owned(), "efgh".to_owned(), 84, 42).is_err());
    assert_eq!(chain_of_bids.get_number_of_auctions(), 2);
}