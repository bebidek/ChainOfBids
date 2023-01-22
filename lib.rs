#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod chain_of_bids {
    #[ink(storage)]
    pub struct ChainOfBids {
    }

    impl ChainOfBids {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { }
        }
        
        #[ink(message)]
        pub fn dummy(&self) {}
    }
}

#[cfg(test)]
mod tests {
    use crate::chain_of_bids::ChainOfBids;
    use ink_lang as ink;

}