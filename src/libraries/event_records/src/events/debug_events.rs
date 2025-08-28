use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct DebugEvent {
    pub details: HashMap<String, String>,
}
