use crate::coordinator_state::CoordinatorState;
use proto::heartbeat::HeartbeatRequest;
use proto::heartbeat::heartbeat_server::Heartbeat;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct HeartbeatInfo {
    pub worker_id: String,
    pub last_heartbeat: prost_types::Timestamp,
    pub num_failed_heartbeats: u32,
}

impl HeartbeatInfo {
    pub fn new(worker_id: String, last_heartbeat: prost_types::Timestamp) -> Self {
        HeartbeatInfo {
            worker_id,
            last_heartbeat,
            num_failed_heartbeats: 0,
        }
    }
}

pub struct HeartbeatService {
    state: Arc<CoordinatorState>,
}

impl HeartbeatService {
    pub fn new(state: Arc<CoordinatorState>) -> Self {
        Self { state }
    }

    pub fn is_worker_alive(&self, worker_id: String) -> bool {
        let heartbeat_info = self.state.heartbeats.get(&worker_id);
        match heartbeat_info {
            Some(info) => {
                let now = prost_types::Timestamp::from(std::time::SystemTime::now());
                let elapsed = now.seconds - info.last_heartbeat.seconds;
            
                // Consider worker alive if last heartbeat was within 10 seconds
                if elapsed < 10 {
                    println!("Worker {} is alive (last heartbeat was {} seconds ago)", worker_id, elapsed);
                    return true;
                } else {
                    println!("Worker {} is considered dead (last heartbeat was {} seconds ago)", worker_id, elapsed);
                    return false;
                }
            }
            None => false,
        }
    }
}

#[tonic::async_trait]
impl Heartbeat for HeartbeatService {
    async fn heartbeat(&self, request: Request<HeartbeatRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        println!("Received heartbeat from worker: {}", req.worker_id);

        // Validate that the worker is registered
        if !self.state.registered_workers.contains_key(&req.worker_id) {
            println!("Received heartbeat from unregistered worker: {}", req.worker_id);
            return Err(Status::not_found("Worker not registered"));
        }

        let heartbeat_info =
            HeartbeatInfo::new(req.worker_id.clone(), prost_types::Timestamp::default());

        match self
            .state
            .heartbeats
            .insert(req.worker_id.clone(), heartbeat_info)
        {
            Some(_) => println!("Updated heartbeat for worker: {}", req.worker_id),
            None => println!("Inserted new heartbeat for worker: {}", req.worker_id),
        }

        Ok(Response::new(()))
    }
}
