use std::cell::RefCell;
use std::time::Duration;
use ic_cdk_timers::TimerId;

use crate::services::strategy_history_service::initialize_strategy_states_and_create_snapshots;

thread_local! {
    static STRATEGY_HISTORY_FETCHING_TIMER_ID: RefCell<Option<TimerId>> = RefCell::new(None);
}

fn set_timer_interval(
    interval: Duration,
    func: impl FnMut() + 'static,
) -> TimerId {
    ic_cdk_timers::set_timer_interval(interval, func)
}

pub fn start_fetching_timer(interval: u64) {
    let timer_id = set_timer_interval(Duration::from_secs(interval), || {
        ic_cdk::spawn(async {
            let _ = initialize_strategy_states_and_create_snapshots().await;
        });
    });

    STRATEGY_HISTORY_FETCHING_TIMER_ID.with(|cell| {
        cell.replace(Some(timer_id));
    });
}

pub fn stop_fetching_timer() {
    STRATEGY_HISTORY_FETCHING_TIMER_ID.with(|timer_id| {
        if let Some(timer_id) = timer_id.borrow_mut().take() {
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}
