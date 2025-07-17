use std::cell::RefCell;
use candid::{Nat, Principal};
use ic_cdk::api::time;
use ic_cdk::caller;

use types::CanisterId;
use types::context::Context;
use errors::internal_error::error::InternalError;
use ::utils::util::nat_to_u64;

use crate::types::types::StrategyId;
use crate::utils::service_resolver::get_service_resolver;

thread_local! {
    pub static USER_ACCOUNTS: RefCell<Vec<UserAccount>> = RefCell::new(Default::default());
}

struct UserAccount {
    user_id: Principal,
    deposits: Vec<UserDeposit>
}

#[allow(unused)]
struct UserDeposit {
    amount: Nat,
    strategy: StrategyId,
    ledger: CanisterId,
    block_index: u64,
    timestamp: u64
}

pub async fn accept_deposit(
    context: Context,
    amount: Nat,
    ledger: Principal,
    strategy_id: StrategyId
) -> Result<(), InternalError> {

    let service_resolver = get_service_resolver();
    let icrc_ledger_client = service_resolver.icrc_ledger_client();

    let block_index = icrc_ledger_client.icrc2_transfer_from(
        context.user.unwrap(),
        ledger,
        amount.clone()
    ).await?;

    let deposit = UserDeposit {
        amount,
        strategy: strategy_id,
        ledger: ledger.into(),
        block_index: nat_to_u64(&block_index),
        timestamp: time()
    };

    USER_ACCOUNTS.with(|accounts| {
        let mut accounts = accounts.borrow_mut();
        let index = accounts.iter().position(|a| a.user_id == caller());
        if let Some(index) = index {
            accounts[index].deposits.push(deposit);
        } else {
            accounts.push(UserAccount {
                user_id: caller(),
                deposits: vec![deposit]
            });
        }
    });

    Ok(())
}
