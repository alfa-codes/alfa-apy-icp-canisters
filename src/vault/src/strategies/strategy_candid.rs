use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::strategies::r#impl::ck_btc_ck_usdt_strategy::ckBTCckUSDTStrategy;
use crate::strategies::r#impl::ck_btc_strategy::ckBTCStrategy;
use crate::strategies::r#impl::panda_icp_stategy::PandaTestStrategy;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use crate::strategies::r#impl::icp_ck_usdt_strategy::IcpCkUSDTStrategy;
use crate::strategies::r#impl::ics_icp_strategy::IcsStrategy;
use crate::strategies::r#impl::icp_ck_eth_strategy::IcpCkETHStrategy;
use crate::strategies::r#impl::ck_btc_icp_strategy::CkBtcIcpStrategy;
use crate::strategies::r#impl::gldt_ck_usdt_strategy::GldtCkUsdtStrategy;
use crate::strategies::strategy::IStrategy;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum StrategyCandid {
    #[allow(non_camel_case_types)]
    ckBTCStrategyV(ckBTCStrategy),
    ICPStrategyV(ICPStrategy),
    #[allow(non_camel_case_types)]
    PandaTestStrategyV(PandaTestStrategy),
    IcpCkUSDTStrategyV(IcpCkUSDTStrategy),
    #[allow(non_camel_case_types)]
    IcsStrategyV(IcsStrategy),
    #[allow(non_camel_case_types)]
    ckBTCckUSDTStrategyV(ckBTCckUSDTStrategy),
    #[allow(non_camel_case_types)]
    IcpCkETHStrategyV(IcpCkETHStrategy),
    #[allow(non_camel_case_types)]
    CkBtcIcpStrategyV(CkBtcIcpStrategy),
    #[allow(non_camel_case_types)]
    GldtCkUsdtStrategyV(GldtCkUsdtStrategy),
}

pub trait Candid {
    fn to_strategy(&self) -> Box<dyn IStrategy>;
}

//TODO maybe move to/from candid object + builders
impl Candid for StrategyCandid {
    fn to_strategy(&self) -> Box<dyn IStrategy> {
        match self {
            StrategyCandid::ckBTCStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::ICPStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::PandaTestStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::IcpCkUSDTStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::IcsStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::ckBTCckUSDTStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::IcpCkETHStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::CkBtcIcpStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::GldtCkUsdtStrategyV(strategy) => Box::new(strategy.clone()),
        }
    }
}