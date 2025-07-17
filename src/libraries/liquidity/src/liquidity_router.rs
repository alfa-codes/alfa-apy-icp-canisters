use std::sync::Arc;

use types::exchange_id::ExchangeId;
use types::CanisterId;
use utils::constants::KONGSWAP_CANISTER_ID;
use service_resolver::ProviderImpls;
use icrc_ledger_client::ICRCLedgerClient;

use crate::clients::kongswap::KongSwapLiquidityClient;
use crate::clients::icpswap::ICPSwapLiquidityClient;
use crate::liquidity_client::LiquidityClient;

pub async fn get_liquidity_client(
    provider_impls: ProviderImpls,
    icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
    token0: CanisterId,
    token1: CanisterId,
    provider: ExchangeId,
) -> Box<dyn LiquidityClient + 'static> {
    match provider {
        ExchangeId::KongSwap => Box::new(
            KongSwapLiquidityClient::new(
                provider_impls.clone(),
                icrc_ledger_client,
                *KONGSWAP_CANISTER_ID,
                token0.clone(), 
                token1.clone()
            )
        ),
        ExchangeId::ICPSwap => Box::new(
            ICPSwapLiquidityClient::new(
                provider_impls,
                icrc_ledger_client,
                token0.clone(), 
                token1.clone()
            ).with_pool().await.unwrap() // TODO: handle error
        ),
        _ => panic!("Unsupported provider"),
    }
}
