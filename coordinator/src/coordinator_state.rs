use crate::apis::heartbeat::HeartbeatInfo;
use crate::apis::registration::RegistrationInfo;
use dashmap::DashMap;

pub struct CoordinatorState {
    pub registered_workers: DashMap<String, RegistrationInfo>,
    pub heartbeats: DashMap<String, HeartbeatInfo>,
}

impl CoordinatorState {
    pub fn new() -> Self {
        Self {
            registered_workers: DashMap::new(),
            heartbeats: DashMap::new(),
        }
    }
}
