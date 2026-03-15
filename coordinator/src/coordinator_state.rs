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

    pub fn is_worker_alive(&self, worker_id: String) -> bool {
        let heartbeat_info = self.heartbeats.get(&worker_id);
        match heartbeat_info {
            Some(info) => {
                let now = prost_types::Timestamp::from(std::time::SystemTime::now());
                let elapsed = now.seconds - info.last_heartbeat.seconds;
            
                // Consider worker alive if last heartbeat was within 10 seconds
                if elapsed < 10 {
                    println!("Worker {} is alive (last heartbeat was {} seconds ago - datetime {})", worker_id, elapsed, info.last_heartbeat);
                    return true;
                } else {
                    println!("Worker {} is considered dead (last heartbeat was {} seconds ago - datetime {})", worker_id, elapsed, info.last_heartbeat);
                    return false;
                }
            }
            None => false, // No heartbeat info found for this worker
        }
    }
}
