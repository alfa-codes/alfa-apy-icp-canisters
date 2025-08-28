use candid::{Principal, Nat, CandidType};
use ic_cdk::id;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use ::types::CanisterId;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    external_services as external_services_area,
    external_services::domains::icrc_ledger as icrc_ledger_domain,
    external_services::domains::icrc_ledger::components as icrc_ledger_domain_components,
};

use icrc_ledger_canister::icrc2_approve::ApproveArgs;
use icrc_ledger_canister::updates::icrc2_transfer_from::Args as Icrc2TransferFromArgs;
use icrc_ledger_types::icrc1::account::Account;

pub mod mock;

// Module code: "01-03-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    external_services_area::AREA_CODE,  // Area code: "01"
    icrc_ledger_domain::DOMAIN_CODE,    // Domain code: "03"
    icrc_ledger_domain_components::CORE // Component code: "01"
);

#[async_trait::async_trait]
pub trait ICRCLedgerClient: Send + Sync + Debug {
    async fn icrc1_decimals(&self, canister_id: CanisterId) -> Result<u8, InternalError>;
    async fn icrc1_fee(&self, canister_id: CanisterId) -> Result<Nat, InternalError>;
    async fn icrc2_approve(
        &self, spender: Principal,
        canister_id: CanisterId,
        amount: Nat
    ) -> Result<Nat, InternalError>;
    async fn icrc2_transfer_from(
        &self,
        from: Principal,
        canister_id: CanisterId,
        amount: Nat
    ) -> Result<Nat, InternalError>;
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct DefaultICRCLedgerClient;

#[async_trait::async_trait]
impl ICRCLedgerClient for DefaultICRCLedgerClient {
    async fn icrc1_decimals(&self, canister_id: CanisterId) -> Result<u8, InternalError> {
        icrc_ledger_canister_c2c_client::icrc1_decimals(canister_id)
            .await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 1), // Error code: "01-03-01 04 01"
                    "icrc_ledger_client::icrc1_decimals".to_string(),
                    format!("IC error calling 'icrc_ledger_canister_c2c_client::icrc1_decimals': {error:?}"),
                    errors::error_extra! {
                        "canister_id" => canister_id.to_text(),
                    }
                )
            })
    }

    async fn icrc2_approve(
        &self,
        spender: Principal,
        canister_id: CanisterId,
        amount: Nat
    ) -> Result<Nat, InternalError> {
        let fee = self.icrc1_fee(canister_id.clone()).await?;
        let approve_amount = amount.clone() + fee.clone();

        let args = ApproveArgs {
            from_subaccount: None,
                spender: spender.into(),
                amount: approve_amount.clone(),
                expected_allowance: None,
                expires_at: None,
                fee: None,
                memo: None,
                created_at_time: None,
        };

        icrc_ledger_canister_c2c_client::icrc2_approve(
            canister_id.clone(),
            &args,
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 2), // Error code: "01-03-01 04 02"
                    "icrc_ledger_client::icrc2_approve".to_string(),
                    format!("IC error calling 'icrc_ledger_canister_c2c_client::icrc2_approve': {error:?}"),
                    errors::error_extra! {
                        "spender" => spender.to_text(),
                        "canister_id" => canister_id.to_text(),
                        "amount" => amount,
                        "fee" => fee,
                        "approve_amount" => approve_amount,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 3), // Error code: "01-03-01 03 03"
                    "icrc_ledger_client::icrc2_approve".to_string(),
                    format!("Error calling 'icrc_ledger_canister_c2c_client::icrc2_approve': {error:?}"),
                    errors::error_extra! {
                        "spender" => spender.to_text(),
                        "canister_id" => canister_id.to_text(),
                        "amount" => amount,
                        "fee" => fee,
                        "approve_amount" => approve_amount,
                    }
                )
            })
    }

    async fn icrc2_transfer_from(
        &self,
        from: Principal,
        canister_id: CanisterId,
        amount: Nat,
    ) -> Result<Nat, InternalError> {
        let args = Icrc2TransferFromArgs {
            spender_subaccount: None,
            from: Account { owner: from, subaccount: None },
            to: Account { owner: id(), subaccount: None },
            amount: amount.clone(),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        icrc_ledger_canister_c2c_client::icrc2_transfer_from(
            canister_id.clone(),
            &args,
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 4), // Error code: "01-03-01 04 04"
                    "icrc_ledger_client::icrc2_transfer_from".to_string(),
                    format!("IC error calling 'icrc_ledger_canister_c2c_client::icrc2_transfer_from': {error:?}"),
                    errors::error_extra! {
                        "from" => from.to_text(),
                        "canister_id" => canister_id.to_text(),
                        "amount" => amount,
                    }
                )
            })?
            .map_err(|err| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 5), // Error code: "01-03-01 03 05"
                    "icrc_ledger_client::icrc2_transfer_from".to_string(),
                    format!("Error calling 'icrc_ledger_canister_c2c_client::icrc2_transfer_from': {err:?}"),
                    errors::error_extra! {
                        "from" => from.to_text(),
                        "canister_id" => canister_id.to_text(),
                        "amount" => amount,
                    }
                )
            })
            .map(|block_index| block_index.0.try_into().unwrap())
    }

    async fn icrc1_fee(&self, canister_id: CanisterId) -> Result<Nat, InternalError> {
        icrc_ledger_canister_c2c_client::icrc1_fee(canister_id)
            .await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 6), // Error code: "01-03-01 04 06"
                    "icrc_ledger_client::icrc1_fee".to_string(),
                    format!("IC error calling 'icrc_ledger_canister_c2c_client::icrc1_fee': {error:?}"),
                    errors::error_extra! {
                        "canister_id" => canister_id.to_text(),
                    }
                )
            })
    }
}
