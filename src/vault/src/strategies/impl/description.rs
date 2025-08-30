use lazy_static::lazy_static;
use std::collections::HashMap;

use types::exchange_id::ExchangeId;
use types::strategies::{StrategyId, Pool};
use types::pool::PoolTrait;
use types::CanisterId;
use utils::constants::{
    CKBTC_TOKEN_CANISTER_ID,
    CKETH_TOKEN_CANISTER_ID,
    CKUSDT_TOKEN_CANISTER_ID,
    ICP_TOKEN_CANISTER_ID,
    ICS_TOKEN_CANISTER_ID,
    PANDA_TOKEN_CANISTER_ID,
    GLDT_TOKEN_CANISTER_ID,
    CKLINK_TOKEN_CANISTER_ID,
};

#[derive(Debug, Clone)]
pub struct StrategyInfo {
    pub name: String,
    pub description: String,
    pub base_token: CanisterId,
    pub pools: Vec<Pool>,
}

// TODO: init from file
lazy_static! {
    pub static ref STRATEGY_MAP: HashMap<StrategyId, StrategyInfo> = {
        let mut strategy_map = HashMap::new();

        strategy_map.insert(1, StrategyInfo {
            name: "ckBTC Growth Strategy".to_string(),
            description: "An aggressive strategy leveraging Kongswap with 50% ckBTC and 50% other assets, including pool pairs like ckBTC/ICP and ckBTC/ckUSDT.".to_string(),
            base_token: *CKBTC_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *CKBTC_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *CKBTC_TOKEN_CANISTER_ID,
                    *CKUSDT_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
            ],
        });

        strategy_map.insert(2, StrategyInfo {
            name: "ICP Stability Strategy".to_string(),
            description: "A balanced strategy utilizing Kongswap with 50% ICP and 50% stable coin, featuring pool pairs like ckUSDC/ICP and ICP/ckUSDT.".to_string(),
            base_token: *ICP_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *ICP_TOKEN_CANISTER_ID,
                    *CKUSDT_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *CKUSDT_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
            ],
        });

        strategy_map.insert(3, StrategyInfo {
            name: "ICP-ckUSDT Dynamic Strategy".to_string(),
            description: "A dynamic strategy that moves the ICP-ckBTC pool between Kongswap and ICPSwap to optimize returns.".to_string(),
            base_token: *ICP_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *ICP_TOKEN_CANISTER_ID,
                    *CKUSDT_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *CKUSDT_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });

        strategy_map.insert(4, StrategyInfo {
            name: "Panda-ICP Balanced Strategy".to_string(),
            description: "A balanced strategy that maintains equal exposure to Panda and ICP tokens across both KongSwap and ICPSwap exchanges for optimal liquidity distribution.".to_string(),
            base_token: *PANDA_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *PANDA_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *PANDA_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });

        strategy_map.insert(5, StrategyInfo {
            name: "ICS-ICP Balanced Strategy".to_string(),
            description: "A balanced strategy that diversifies exposure between ICS and ICP tokens across KongSwap and ICPSwap exchanges, providing stable returns through cross-exchange arbitrage opportunities.".to_string(),
            base_token: *ICS_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *ICS_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *ICS_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });

        strategy_map.insert(6, StrategyInfo {
            name: "ckBTC-ckUSDT Balanced Strategy".to_string(),
            description: "A balanced strategy that maintains stable exposure to ckBTC while providing liquidity to ckUSDT pairs across KongSwap and ICPSwap exchanges for consistent returns.".to_string(),
            base_token: *CKBTC_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *CKBTC_TOKEN_CANISTER_ID,
                    *CKUSDT_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *CKBTC_TOKEN_CANISTER_ID,
                    *CKUSDT_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });

        strategy_map.insert(7, StrategyInfo {
            name: "ICP-ckETH Dynamic Strategy".to_string(),
            description: "A dynamic strategy that actively manages ICP and ckETH positions across KongSwap and ICPSwap exchanges, optimizing for yield through cross-exchange liquidity provision and rebalancing.".to_string(),
            base_token: *ICP_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *CKETH_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *ICP_TOKEN_CANISTER_ID,
                    *CKETH_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });

        strategy_map.insert(8, StrategyInfo {
            name: "ckBTC-ICP Dynamic Strategy".to_string(),
            description: "A dynamic strategy that actively manages ckBTC and ICP positions across KongSwap and ICPSwap exchanges, optimizing for yield through cross-exchange liquidity provision and strategic rebalancing based on market conditions.".to_string(),
            base_token: *CKBTC_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *CKBTC_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *CKBTC_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });

        strategy_map.insert(9, StrategyInfo {
            name: "GLDT-ckUSDT Balanced Strategy".to_string(),
            description: "A balanced strategy that maintains stable exposure to GLDT while providing liquidity to ckUSDT pairs across KongSwap and ICPSwap exchanges, designed for consistent returns in the gold-backed token market.".to_string(),
            base_token: *GLDT_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *GLDT_TOKEN_CANISTER_ID,
                    *CKUSDT_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *GLDT_TOKEN_CANISTER_ID,
                    *CKUSDT_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });

        strategy_map.insert(10, StrategyInfo {
            name: "CKLINK-ICP Balanced Strategy".to_string(),
            description: "A balanced strategy that maintains stable exposure to CKLINK while providing liquidity to ICP pairs across KongSwap and ICPSwap exchanges, designed for consistent returns in the LINK-backed token market.".to_string(),
            base_token: *CKLINK_TOKEN_CANISTER_ID,
            pools: vec![
                Pool::build(
                    *CKLINK_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *CKLINK_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });

        strategy_map
    };
}
