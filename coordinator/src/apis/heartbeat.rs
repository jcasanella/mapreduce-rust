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
}

#[tonic::async_trait]
impl Heartbeat for HeartbeatService {
    async fn heartbeat(&self, request: Request<HeartbeatRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        println!("Received heartbeat from worker: {}", req.worker_id);

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
