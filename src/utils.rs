use near_sdk::{env, log, require, Balance, Promise, StorageUsage};

pub(crate) fn prefix_key(prefix: &Vec<u8>, key: &[u8]) -> Vec<u8> {
  [prefix as &[u8], key].concat()
}

pub(crate) fn storage_refund(storage_usage_start: StorageUsage, other_fees: Balance) {
  let storage_usage_end = env::storage_usage();

  let storage_fee =
    Balance::from(storage_usage_end.saturating_sub(storage_usage_start)) * env::storage_byte_cost();

  let total_required_deposit = storage_fee + other_fees;

  let attached_deposit = env::attached_deposit();

  require!(
    attached_deposit >= total_required_deposit,
    format!(
      "Insufficient deposit: required: {} yoctoNEAR; received: {} yoctoNEAR",
      &total_required_deposit, &attached_deposit
    )
  );

  let refund = attached_deposit - total_required_deposit;

  log!("storage fee: {} yoctoNEAR", &storage_fee);
  log!("refund: {} yoctoNEAR", &refund);

  if refund > 0 {
    Promise::new(env::predecessor_account_id()).transfer(refund);
  }
}
