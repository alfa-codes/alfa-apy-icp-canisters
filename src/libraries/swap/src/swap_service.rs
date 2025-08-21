use candid::Nat;
use std::collections::HashMap;
use std::sync::Arc;

use types::swap_tokens::{SwapResponse, QuoteResponse};
use types::exchange_id::ExchangeId;
use utils::constants::KONGSWAP_CANISTER_ID;
use types::CanisterId;
use icrc_ledger_client::ICRCLedgerClient;
use providers::kongswap::KongSwapProvider;
use providers::icpswap::ICPSwapProvider;
use service_resolver::ProviderImpls;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    libraries as library_area,
    libraries::domains::swap as swap_domain,
    libraries::domains::swap::components as swap_domain_components,
};

use crate::token_swaps::kongswap::KongSwapSwapClient;
use crate::token_swaps::icpswap::ICPSwapSwapClient;
use crate::token_swaps::swap_client::SwapClient;

// Module code: "02-01-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    library_area::AREA_CODE,             // Area code: "02"
    swap_domain::DOMAIN_CODE,            // Domain code: "01"
    swap_domain_components::SWAP_SERVICE // Component code: "01"
);

pub async fn swap_icrc2_optimal(
    provider_impls: ProviderImpls,
    icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<SwapResponse, InternalError> {
    let provider = quote_swap_icrc2_optimal(
        provider_impls.clone(),
        input_token.clone(),
        output_token.clone(),
        amount.clone()
    ).await?.provider;

    swap_icrc2(
        provider_impls,
        icrc_ledger_client,
        input_token,
        output_token,
        amount,
        provider
    ).await
}

pub async fn swap_icrc2(
    provider_impls: ProviderImpls,
    icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
    provider: ExchangeId,
) -> Result<SwapResponse, InternalError>
{
    match provider {
        ExchangeId::KongSwap => {
            swap_icrc2_kongswap(
                provider_impls.kongswap,
                icrc_ledger_client,
                input_token,
                output_token,
                amount
            ).await
        }
        ExchangeId::ICPSwap => {
            swap_icrc2_icpswap(
                provider_impls.icpswap,
                icrc_ledger_client,
                input_token,
                output_token,
                amount
            ).await
        }
        _ => Err(InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 1), // Error code: "02-01-01 03 01"
            "swap_service::swap_icrc2".to_string(),
            "Invalid provider".to_string(),
            errors::error_extra! {
                "input_token" => input_token,
                "output_token" => output_token,
                "amount" => amount,
                "provider" => provider,
            },
        )),
    }
}

pub async fn quote_swap_icrc2_optimal(
    provider_impls: ProviderImpls,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<QuoteResponse, InternalError>
{
    let kong_quote = quote_swap_kongswap(
        provider_impls.kongswap,
        input_token.clone(),
        output_token.clone(),
        amount
    ).await;
    // let icp_quote = quote_swap_icpswap(
    //     icpswap_provider,
    //     input_token.clone(),
    //     output_token.clone(),
    //     amount
    // ).await;

    //Return the quote with the highest amount_out
    // std::cmp::max_by(
    //     kong_quote.unwrap(),
    //     icp_quote.unwrap(),
    //     |a, b| a.amount_out.cmp(&b.amount_out)
    // )

    // TODO: remove this after testing and return the quote with the highest amount_out
    Ok(kong_quote?)
}

pub async fn quote_swap_icrc2(
    provider_impls: ProviderImpls,
    icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
    provider: ExchangeId,
) -> Result<QuoteResponse, InternalError>
{
    match provider {
        ExchangeId::KongSwap => {
            quote_swap_kongswap(
                provider_impls.kongswap,
                input_token,
                output_token,
                amount
            ).await
        }
        ExchangeId::ICPSwap => {
            quote_swap_icpswap(
                provider_impls.icpswap,
                icrc_ledger_client,
                input_token,
                output_token,
                amount
            ).await
        }
        _ => Err(InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 2), // Error code: "02-01-01 03 02"
            "swap_service::quote_swap_icrc2".to_string(),
            "Invalid provider".to_string(),
            errors::error_extra! {
                "input_token" => input_token,
                "output_token" => output_token,
                "amount" => amount,
                "provider" => provider,
            },
        )),
    }
}

// TODO: make private
pub async fn swap_icrc2_kongswap(
    provider_impl: Arc<dyn KongSwapProvider + Send + Sync>,
    icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<SwapResponse, InternalError>
{
    let swap_client = Box::new(
        KongSwapSwapClient::new(
            provider_impl,
            *KONGSWAP_CANISTER_ID,
            input_token.clone(),
            output_token
        )
    );

    icrc_ledger_client.icrc2_approve(
        swap_client.canister_id(),
        input_token.clone(),
        amount.clone()
    ).await?;

    let swap_result = swap_client.swap(amount.clone()).await?;

    Ok(SwapResponse {
        provider: ExchangeId::KongSwap,
        amount_out: swap_result.amount_out,
    })
}

// TODO: make private
pub async fn swap_icrc2_icpswap(
    provider_impl: Arc<dyn ICPSwapProvider + Send + Sync>,
    icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<SwapResponse, InternalError>
{
    let swap_client = Box::new(
        ICPSwapSwapClient::new(
            provider_impl,
            icrc_ledger_client.clone(),
            input_token.clone(),
            output_token
        ).with_pool().await?
    );

    icrc_ledger_client.icrc2_approve(
        swap_client.canister_id(),
        input_token.clone(),
        amount.clone()
    ).await?;

    let swap_result = swap_client.swap(amount.clone()).await?;

    Ok(SwapResponse {
        provider: ExchangeId::ICPSwap,
        amount_out: swap_result.amount_out,
    })
}

// TODO: make private
pub async fn quote_swap_kongswap(
    provider_impl: Arc<dyn KongSwapProvider + Send + Sync>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<QuoteResponse, InternalError>
{
    let swap_client = Box::new(
        KongSwapSwapClient::new(
            provider_impl,
            *KONGSWAP_CANISTER_ID,
            input_token.clone(),
            output_token.clone()
        )
    );

    let result = swap_client.quote(amount.clone()).await?;

    Ok(QuoteResponse {
        provider: ExchangeId::KongSwap,
        amount_out: result.amount_out,
    })
}

// TODO: make private
pub async fn quote_swap_icpswap(
    provider_impl: Arc<dyn ICPSwapProvider + Send + Sync>,
    icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<QuoteResponse, InternalError>
{
    let swap_client = Box::new(
        ICPSwapSwapClient::new(
            provider_impl,
            icrc_ledger_client,
            input_token.clone(),
            output_token.clone()
        ).with_pool().await?
    );

    let result = swap_client.quote(amount.clone()).await?;

    Ok(QuoteResponse {
        provider: ExchangeId::ICPSwap,
        amount_out: result.amount_out,
    })
}
