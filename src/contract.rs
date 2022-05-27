use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, BorshStorageKey, PanicOnDefault,
};

#[allow(unused)]
#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Item,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {}
    }
}
