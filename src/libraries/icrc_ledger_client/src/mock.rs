use candid::{Nat, Principal, CandidType};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use ::types::CanisterId;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    external_services as external_services_area,
    external_services::domains::icrc_ledger as icrc_ledger_domain,
    external_services::domains::icrc_ledger::components as icrc_ledger_domain_components,
};

use crate::ICRCLedgerClient;

// Module code: "01-03-51"
errors::define_error_code_builder_fn!(
    build_error_code,
    external_services_area::AREA_CODE,       // Area code: "01"
    icrc_ledger_domain::DOMAIN_CODE,         // Domain code: "03"
    icrc_ledger_domain_components::MOCK_CORE // Component code: "51"
);

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
                build_error_code(InternalErrorKind::NotFound, 1), // Error code: "01-03-51 01 01"
                "MockICRCLedgerClient::icrc1_decimals".to_string(),
                "Mock response not set for decimals".to_string(),
                errors::error_extra! {
                    "canister_id" => canister_id,
                }
            ))
        })
    }

    async fn icrc2_approve(
        &self,
        spender: Principal,
        canister_id: CanisterId,
        amount: Nat
    ) -> Result<Nat, InternalError> {
        self.approve_responses
            .get(&(spender.to_text(), canister_id.to_text(), amount.to_string()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 2), // Error code: "01-03-51 01 02"
                    "MockICRCLedgerClient::icrc2_approve".to_string(),
                    "Mock response not set for approve".to_string(),
                    errors::error_extra! {
                        "spender" => spender,
                        "amount" => amount,
                    }
                )),
                |r| r.to_owned()
            )
    }

    async fn icrc2_transfer_from(
        &self,
        from: Principal,
        canister_id: CanisterId,
        amount: Nat
    ) -> Result<Nat, InternalError> {
        self.transfer_from_responses
            .get(&(from.to_text(), canister_id.to_text(), amount.to_string()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 3), // Error code: "01-03-51 01 03"
                    "MockICRCLedgerClient::icrc2_transfer_from".to_string(),
                    "Mock response not set for transfer_from".to_string(),
                    errors::error_extra! {
                        "from" => from,
                        "amount" => amount,
                    }
                )),
                |r| r.to_owned()
            )
    }

    async fn icrc1_fee(&self, canister_id: CanisterId) -> Result<Nat, InternalError> {
        self.fee_responses.get(&canister_id).cloned().unwrap_or_else(|| {
            Err(InternalError::not_found(
                build_error_code(InternalErrorKind::NotFound, 4), // Error code: "01-03-51 01 04"
                "MockICRCLedgerClient::icrc1_fee".to_string(),
                "Mock response not set for fee".to_string(),
                errors::error_extra! {
                    "canister_id" => canister_id,
                }
            ))
        })
    }
}
