#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        /// Stores a single `bool` value on the storage.
        value: i32,
        my_number_map: ink_storage::collections::HashMap<AccountId, i32>,
        owner: AccountId,
    }

    impl Incrementer {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                my_number_map: Default::default(),
                owner: Self::env().caller(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(0)
        }

        /// Simply returns the current value of our `i32`.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn my_setter(&mut self, new_value: i32) {
            self.value = new_value;
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) -> i32 {
            let incr = self.value + by;
            self.value = incr;
            return incr;
        }

        /// Private function for our hashmap
        /// Returns the number for an accountId or 0 if it has not been set
        fn my_number_or_zero(&self, of: &AccountId) -> i32 {
            let balance = self.my_number_map.get(of).unwrap_or(&0);
            *balance
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller = self.owner;
            self.my_number_or_zero(&caller)
        }

        #[ink(message)]
        pub fn set_my_number(&mut self, value: i32) {
            self.my_number_map.insert(self.owner, value);
        }

        #[link(message)]
        pub fn add_my_number(&mut self, value: i32) {
            let my_number = self.my_number_or_zero(&self.owner);
            self.my_number_map.insert(self.owner, my_number + value);
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut incrementer = Incrementer::new(0);
            assert_eq!(incrementer.get(), 0);
            incrementer.inc(10);
            assert_eq!(incrementer.get(), 10);
        }
    }
}
