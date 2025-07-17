use candid::{Nat, Principal};
use errors::internal_error::error::{InternalError, build_error_code};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use candid::CandidType;

use ::types::CanisterId;
use crate::ICRCLedgerClient;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct MockICRCLedgerClient {
    decimals_responses: HashMap<CanisterId, Result<u8, InternalError>>,
    approve_responses: HashMap<(String, String, String), Result<Nat, InternalError>>,
    transfer_from_responses: HashMap<(String, String, String), Result<Nat, InternalError>>,
    fee_responses: HashMap<CanisterId, Result<Nat, InternalError>>,
}

impl Default for MockICRCLedgerClient {
    fn default() -> Self {
        Self {
            decimals_responses: HashMap::new(),
            approve_responses: HashMap::new(),
            transfer_from_responses: HashMap::new(),
            fee_responses: HashMap::new(),
        }
    }
}

impl MockICRCLedgerClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mock_decimals(&mut self, canister_id: CanisterId, response: Result<u8, InternalError>) {
        self.decimals_responses.insert(canister_id, response);
    }

    pub fn mock_approve(
        &mut self,
        spender: Principal,
        canister_id: CanisterId,
        amount: Nat,
        response: Result<Nat, InternalError>,
    ) {
        self.approve_responses.insert(
            (spender.to_text(), canister_id.to_text(), amount.to_string()),
            response
        );
    }

    pub fn mock_transfer_from(
        &mut self,
        from: Principal,
        canister_id: CanisterId,
        amount: Nat,
        response: Result<Nat, InternalError>,
    ) {
        self.transfer_from_responses.insert(
            (from.to_text(), canister_id.to_text(), amount.to_string()),
            response
        );
    }

    pub fn mock_fee(&mut self, canister_id: CanisterId, response: Result<Nat, InternalError>) {
        self.fee_responses.insert(canister_id, response);
    }
}

#[async_trait::async_trait]
impl ICRCLedgerClient for MockICRCLedgerClient {
    async fn icrc1_decimals(&self, canister_id: CanisterId) -> Result<u8, InternalError> {
        self.decimals_responses.get(&canister_id).cloned().unwrap_or_else(|| {
            Err(InternalError::not_found(
                build_error_code(0000, 0, 0),
                "MockICRCLedgerClient::icrc1_decimals".to_string(),
                "Mock response not set for decimals".to_string(),
                Some(HashMap::from([("canister_id".to_string(), canister_id.to_text())]))
            ))
        })
    }

    async fn icrc2_approve(&self, spender: Principal, canister_id: CanisterId, amount: Nat) -> Result<Nat, InternalError> {
        self.approve_responses
            .get(&(spender.to_text(), canister_id.to_text(), amount.to_string()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(0000, 0, 0),
                    "MockICRCLedgerClient::icrc2_approve".to_string(),
                    "Mock response not set for approve".to_string(),
                    Some(HashMap::from([
                        ("spender".to_string(), spender.to_text()),
                        ("amount".to_string(), amount.to_string()),
                    ]))
                )),
                |r| r.to_owned()
            )
    }

    async fn icrc2_transfer_from(&self, from: Principal, canister_id: CanisterId, amount: Nat) -> Result<Nat, InternalError> {
        self.transfer_from_responses
            .get(&(from.to_text(), canister_id.to_text(), amount.to_string()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(0000, 0, 0),
                    "MockICRCLedgerClient::icrc2_transfer_from".to_string(),
                    "Mock response not set for transfer_from".to_string(),
                    Some(HashMap::from([
                        ("from".to_string(), from.to_text()),
                        ("amount".to_string(), amount.to_string()),
                    ]))
                )),
                |r| r.to_owned()
            )
    }

    async fn icrc1_fee(&self, canister_id: CanisterId) -> Result<Nat, InternalError> {
        self.fee_responses.get(&canister_id).cloned().unwrap_or_else(|| {
            Err(InternalError::not_found(
                build_error_code(0000, 0, 0),
                "MockICRCLedgerClient::icrc1_fee".to_string(),
                "Mock response not set for fee".to_string(),
                Some(HashMap::from([("canister_id".to_string(), canister_id.to_text())]))
            ))
        })
    }
}
