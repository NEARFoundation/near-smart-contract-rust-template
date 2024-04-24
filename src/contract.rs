use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen,
    store::Vector,
    BorshStorageKey, PanicOnDefault,
};
#[allow(clippy::wildcard_imports)]
use near_sdk_contract_tools::{event, owner::*, standard::nep297::Event, Owner};

#[allow(unused)]
#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    History,
}

#[event(
    standard = "x-value-history",
    version = "1.0.0",
    serde = "near_sdk::serde"
)]
enum ContractEvent {
    ValueSet { old_value: u32, new_value: u32 },
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize, Owner)]
pub struct Contract {
    value: u32,
    history: Vector<u32>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {
            value: 0,
            history: Vector::new(StorageKey::History),
        };

        let predecessor = env::predecessor_account_id();
        Owner::init(&mut contract, &predecessor);

        contract.history.push(0);

        contract
    }

    pub fn set_value(&mut self, value: u32) {
        self.assert_owner();

        self.history.push(value);
        let old_value = self.value;
        self.value = value;

        ContractEvent::ValueSet {
            old_value,
            new_value: value,
        }
        .emit();
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn get_historical_value(&self, index: u32) -> Option<&u32> {
        self.history
            .len()
            .checked_sub(index + 1)
            .map(|i| self.history.get(i).unwrap())
    }

    pub fn get_history_length(&self) -> u32 {
        self.history.len()
    }
}
