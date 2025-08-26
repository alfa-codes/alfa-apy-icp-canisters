use ::types::strategies::StrategyResponse;

use crate::strategies::r#impl::ck_btc_ck_usdt_strategy::ckBTCckUSDTStrategy;
use crate::strategies::r#impl::ck_btc_strategy::ckBTCStrategy;
use crate::strategies::r#impl::panda_icp_stategy::PandaTestStrategy;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use crate::strategies::r#impl::icp_ck_usdt_strategy::IcpCkUSDTStrategy;
use crate::strategies::r#impl::ics_icp_strategy::IcsStrategy;
use crate::strategies::r#impl::icp_ck_eth_strategy::IcpCkETHStrategy;
use crate::repository::strategies_repo;

pub fn init_strategies() {
    strategies_repo::add_if_not_exists(Box::new(ckBTCStrategy::new()));
    strategies_repo::add_if_not_exists( Box::new(ICPStrategy::new()));
    strategies_repo::add_or_update_strategy(Box::new(IcpCkUSDTStrategy::new()));
    strategies_repo::add_or_update_strategy(Box::new(PandaTestStrategy::new()));
    strategies_repo::add_or_update_strategy(Box::new(IcsStrategy::new()));
    strategies_repo::add_or_update_strategy(Box::new(ckBTCckUSDTStrategy::new()));
    strategies_repo::add_or_update_strategy(Box::new(IcpCkETHStrategy::new()));
}

pub fn get_actual_strategies() -> Vec<StrategyResponse> {
    strategies_repo::get_enabled_strategies()
        .into_iter()
        .map(|s| s.to_response())
        .collect()
}
