#![feature(proc_macro_hygiene)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(
    version = "0.1.0",
    compile_as_dependency = true,
)]
mod flipper {
    #[ink(storage)]
    struct Flipper {
        value: storage::Value<bool>,
    }

    impl Flipper {
        #[ink(constructor)]
        fn new(&mut self, init_value: bool) {
            self.value.set(init_value);
        }

        #[ink(constructor)]
        fn default(&mut self) {
            self.new(false)
        }

        #[ink(message)]
        fn flip(&mut self) {
            *self.value = !self.get();
        }

        #[ink(message)]
        fn get(&self) -> bool {
            *self.value
        }
    }
}

fn main() {}
