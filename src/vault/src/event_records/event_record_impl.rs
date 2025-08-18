use candid::Principal;
use ic_cdk::api::time;

use event_records::generic_event_record::GenericEventRecord;
use types::strategies::StrategyId;

use crate::event_records::event_record::{EventRecord, Event};
use crate::repository::event_records_repo;

impl EventRecord {
    pub fn new(
        id: u64,
        correlation_id: String,
        event: Event,
        timestamp: u64,
        user: Option<Principal>,
        strategy_id: Option<StrategyId>,
    ) -> Self {
        Self(GenericEventRecord {
            id,
            event,
            timestamp,
            correlation_id,
            user,
            strategy_id,
        })
    }

    pub fn build(
        id: u64,
        correlation_id: String,
        event: Event,
        user: Option<Principal>,
        strategy_id: Option<StrategyId>,
    ) -> Self {
        Self::new(
            id,
            correlation_id,
            event,
            time(),
            user,
            strategy_id,
        )
    }

    pub fn save(&self) {
        event_records_repo::save_event_record(self.clone());
    }
}
