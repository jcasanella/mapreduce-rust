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

    pub fn process_heartbeat(&self) {
        let now = prost_types::Timestamp::from(std::time::SystemTime::now());
        let mut to_increment = Vec::new();
        let mut to_remove = Vec::new();
        let mut to_reset = Vec::new();

        println!(
            "Registered workers: {}, Heartbeats tracked: {}",
            self.registered_workers.len(),
            self.heartbeats.len()
        );

        // Phase 1: read-only, collect decisions
        for entry in self.heartbeats.iter() {
            let elapsed = now.seconds - entry.value().last_heartbeat.seconds;
            if elapsed >= 10 {
                if entry.value().num_failed_heartbeats < 3 {
                    // If retries < 3, increment retries
                    to_increment.push(entry.key().clone());
                } else {
                    // Else retries == 3, remove worker from registered workers and heartbeats
                    to_remove.push(entry.key().clone());
                }
            } else {
                // Not older than 10 seconds, reset retries to 0
                to_reset.push(entry.key().clone());
            }
        }

        // Phase 2: mutate freely
        for worker_id in to_increment {
            println!(
                "Worker {} has not sent a heartbeat in the last 10 seconds, incrementing failed heartbeats",
                worker_id
            );
            self.increment_failed_heartbeats(&worker_id);
        }

        for worker_id in to_remove {
            println!(
                "Worker {} has failed to send a heartbeat in the last 10 seconds for 3 consecutive times, removing worker",
                worker_id
            );
            self.registered_workers.remove(&worker_id);
            self.heartbeats.remove(&worker_id);
        }

        for worker_id in to_reset {
            println!(
                "Worker {} has sent a heartbeat in the last 10 seconds, resetting failed heartbeats",
                worker_id
            );
            self.reset_failed_heartbeats(&worker_id);
        }
    }

    fn increment_failed_heartbeats(&self, worker_id: &String) {
        if let Some(mut info) = self.heartbeats.get_mut(worker_id) {
            info.num_failed_heartbeats += 1;
            println!(
                "Incremented failed heartbeats for worker {}: now {}",
                worker_id, info.num_failed_heartbeats
            );
        } else {
            println!(
                "No heartbeat info found for worker {} to increment failed heartbeats",
                worker_id
            );
        }
    }

    fn reset_failed_heartbeats(&self, worker_id: &String) {
        if let Some(mut info) = self.heartbeats.get_mut(worker_id) {
            info.num_failed_heartbeats = 0;
            println!(
                "Reset heartbeats for worker {}: now {}",
                worker_id, info.num_failed_heartbeats
            );
        } else {
            println!(
                "No heartbeat info found for worker {} to reset failed heartbeats",
                worker_id
            );
        }
    }
}

impl Default for CoordinatorState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_heartbeat_failed_processing() {
        let state = CoordinatorState::new();
        let worker_id = "worker1".to_string();
        let registration_info = RegistrationInfo::new(
            worker_id.clone(),
            prost_types::Timestamp::from(std::time::SystemTime::now()),
        );
        state
            .registered_workers
            .insert(worker_id.clone(), registration_info);
        let heartbeat_info = HeartbeatInfo::new(
            worker_id.clone(),
            prost_types::Timestamp::from(
                std::time::SystemTime::now() - std::time::Duration::from_secs(11),
            ),
        );
        state.heartbeats.insert(worker_id.clone(), heartbeat_info);

        // Process heartbeats, should increment failed heartbeats to 1
        state.process_heartbeat();
        assert_eq!(
            state
                .heartbeats
                .get(&worker_id)
                .unwrap()
                .num_failed_heartbeats,
            1
        );
    }

    #[tokio::test]
    async fn test_heartbeat_removal() {
        let state = CoordinatorState::new();
        let worker_id = "worker2".to_string();
        let registration_info = RegistrationInfo::new(
            worker_id.clone(),
            prost_types::Timestamp::from(std::time::SystemTime::now()),
        );
        state
            .registered_workers
            .insert(worker_id.clone(), registration_info);
        let heartbeat_info = HeartbeatInfo::new(
            worker_id.clone(),
            prost_types::Timestamp::from(
                std::time::SystemTime::now() - std::time::Duration::from_secs(11),
            ),
        );
        state.heartbeats.insert(worker_id.clone(), heartbeat_info);

        // Process heartbeats, accept 3 failed heartbeats, then remove worker
        state.process_heartbeat();
        assert_eq!(
            state
                .heartbeats
                .get(&worker_id)
                .unwrap()
                .num_failed_heartbeats,
            1
        );

        state.process_heartbeat();
        assert_eq!(
            state
                .heartbeats
                .get(&worker_id)
                .unwrap()
                .num_failed_heartbeats,
            2
        );

        state.process_heartbeat();
        assert_eq!(
            state
                .heartbeats
                .get(&worker_id)
                .unwrap()
                .num_failed_heartbeats,
            3
        );

        // After 3 failed heartbeats, worker should be removed
        state.process_heartbeat();

        assert!(!state.registered_workers.contains_key(&worker_id));
        assert!(!state.heartbeats.contains_key(&worker_id));
    }

    #[tokio::test]
    async fn test_heartbeat_reset() {
        let state = CoordinatorState::new();
        let worker_id = "worker3".to_string();
        let registration_info = RegistrationInfo::new(
            worker_id.clone(),
            prost_types::Timestamp::from(std::time::SystemTime::now()),
        );
        state
            .registered_workers
            .insert(worker_id.clone(), registration_info);
        let heartbeat_info = HeartbeatInfo::new(
            worker_id.clone(),
            prost_types::Timestamp::from(
                std::time::SystemTime::now() - std::time::Duration::from_secs(11),
            ),
        );
        state.heartbeats.insert(worker_id.clone(), heartbeat_info);

        // Process heartbeats, should increment failed heartbeats to 1
        state.process_heartbeat();
        assert_eq!(
            state
                .heartbeats
                .get(&worker_id)
                .unwrap()
                .num_failed_heartbeats,
            1
        );

        state.process_heartbeat();
        assert_eq!(
            state
                .heartbeats
                .get(&worker_id)
                .unwrap()
                .num_failed_heartbeats,
            2
        );

        // Now simulate a heartbeat within the last 10 seconds, which should reset failed heartbeats to 0
        let new_heartbeat_info = HeartbeatInfo::new(
            worker_id.clone(),
            prost_types::Timestamp::from(std::time::SystemTime::now()),
        );
        state
            .heartbeats
            .insert(worker_id.clone(), new_heartbeat_info);

        state.process_heartbeat();
        assert_eq!(
            state
                .heartbeats
                .get(&worker_id)
                .unwrap()
                .num_failed_heartbeats,
            0
        );
    }
}
