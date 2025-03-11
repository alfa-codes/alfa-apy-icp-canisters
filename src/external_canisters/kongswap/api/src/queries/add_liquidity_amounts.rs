use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};


pub type  Args =  (String, Nat, String);
pub type Response = (Result<AddLiquidityAmountsReply, String>,);

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct AddLiquidityAmountsReply {
    pub symbol: String,
    pub chain_0: String,
    pub address_0: String,
    pub symbol_0: String,
    pub amount_0: Nat,
    pub fee_0: Nat,
    pub chain_1: String,
    pub address_1: String,
    pub symbol_1: String,
    pub amount_1: Nat,
    pub fee_1: Nat,
    pub add_lp_token_amount: Nat,
}

