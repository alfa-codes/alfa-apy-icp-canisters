use ::types::strategies::StrategyResponse;

use crate::strategies::r#impl::ck_btc_ck_usdt_strategy::ckBTCckUSDTStrategy;
use crate::strategies::r#impl::ck_btc_strategy::ckBTCStrategy;
use crate::strategies::r#impl::panda_icp_stategy::PandaTestStrategy;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use crate::strategies::r#impl::icp_ck_usdt_strategy::IcpCkUSDTStrategy;
use crate::strategies::r#impl::ics_icp_strategy::IcsStrategy;
use crate::strategies::r#impl::icp_ck_eth_strategy::IcpCkETHStrategy;
use crate::strategies::r#impl::ck_btc_icp_strategy::CkBtcIcpStrategy;
use crate::strategies::r#impl::gldt_ck_usdt_strategy::GldtCkUsdtStrategy;
use crate::strategies::r#impl::ck_link_icp_strategy::CkLinkIcpStrategy;
use crate::repository::strategies_repo;

pub fn init_strategies() {
    strategies_repo::add_if_not_exists(Box::new(ckBTCStrategy::new()));
    strategies_repo::add_if_not_exists(Box::new(ICPStrategy::new()));
    strategies_repo::add_if_not_exists(Box::new(IcpCkUSDTStrategy::new()));
    strategies_repo::add_if_not_exists(Box::new(PandaTestStrategy::new()));
    strategies_repo::add_if_not_exists(Box::new(IcsStrategy::new()));
    strategies_repo::add_if_not_exists(Box::new(ckBTCckUSDTStrategy::new()));
    strategies_repo::add_if_not_exists(Box::new(IcpCkETHStrategy::new()));
    strategies_repo::add_if_not_exists(Box::new(CkBtcIcpStrategy::new()));
    strategies_repo::add_if_not_exists(Box::new(GldtCkUsdtStrategy::new()));
    strategies_repo::add_if_not_exists(Box::new(CkLinkIcpStrategy::new()));
}

pub fn get_actual_strategies() -> Vec<StrategyResponse> {
    strategies_repo::get_enabled_strategies()
        .into_iter()
        .map(|s| s.to_response())
        .collect()
}
