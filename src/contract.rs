use crate::*;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    ITEM,
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
