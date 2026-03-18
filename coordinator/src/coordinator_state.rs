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

    pub fn increment_failed_heartbeats(&self, worker_id: String) {
        if let Some(mut info) = self.heartbeats.get_mut(&worker_id) {
            info.num_failed_heartbeats += 1;
            println!("Incremented failed heartbeats for worker {}: now {}", worker_id, info.num_failed_heartbeats);
        } else {
            println!("No heartbeat info found for worker {} to increment failed heartbeats", worker_id);
        }

    }

    pub fn reset_failed_heartbeats(&self, worker_id: String) {
        if let Some(mut info) = self.heartbeats.get_mut(&worker_id) {
            info.num_failed_heartbeats = 0;
            println!("Reset failed heartbeats for worker {}: now {}", worker_id, info.num_failed_heartbeats);
        } else {
            println!("No heartbeat info found for worker {} to reset failed heartbeats", worker_id);
        }
    }
}
