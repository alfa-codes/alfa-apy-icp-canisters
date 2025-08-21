use candid::{Nat, Principal};
use std::convert::TryInto;

use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use icrc_ledger_canister::updates::icrc1_transfer::Response as Icrc1TransferResponse;
use canister_client;
use types::CanisterId;

use errors::internal_error::error::{InternalError, InternalErrorKind};

use errors::internal_error::error_codes::module::areas::{
    external_services as external_services_area,
    external_services::domains::canister as canister_domain,
    external_services::domains::canister::components as canister_domain_components,
};


use crate::environment::Environment;

// Module code: "01-04-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    external_services_area::AREA_CODE, // Area code: "01"
    canister_domain::DOMAIN_CODE,      // Domain code: "04"
    canister_domain_components::CORE   // Component code: "01"
);

pub async fn icrc1_transfer_to_user(
    environment: &Environment,
    user: Principal,
    canister_id: CanisterId,
    amount: Nat,
) -> Result<Nat, InternalError> {
    let args = TransferArg {
        from_subaccount: None,
        to: Account { owner: user, subaccount: None },
        fee: None,
        created_at_time: None,
        memo: None,
        amount: amount.clone(),
    };

    if environment.should_use_mock_services() {
        return Ok(amount);
    }

    canister_client::make_c2c_call(
        canister_id,
        "icrc1_transfer",
        &args,
        ::candid::encode_one,
        |r| ::candid::decode_one::<Icrc1TransferResponse>(r)
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(InternalErrorKind::ExternalService, 1), // Error code: "01-04-01 04 01"
                "Utils::icrc1_transfer_to_user".to_string(),
                format!("IC error calling 'canister_client::make_c2c_call': {error:?}"),
                errors::error_extra! {
                    "user" => user,
                    "canister_id" => canister_id,
                    "amount" => amount,
                },
            )
        })?
        .map_err(|err| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 2), // Error code: "01-04-01 03 02"
                "Utils::icrc1_transfer_to_user".to_string(),
                format!("Error calling 'canister_client::make_c2c_call': {err:?}"),
                errors::error_extra! {
                    "user" => user,
                    "canister_id" => canister_id,
                    "amount" => amount,
                },
            )
        })
        .map(|response| response.0.try_into().unwrap())
}
