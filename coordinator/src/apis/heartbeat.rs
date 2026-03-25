use crate::coordinator_state::CoordinatorState;
use proto::heartbeat::{HeartbeatRequest, HeartbeatResponse, HeartbeatAction};
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
    async fn heartbeat(&self, request: Request<HeartbeatRequest>) -> Result<Response<HeartbeatResponse>, Status> {
        let req = request.into_inner();
        println!("Received heartbeat from worker: {}", req.worker_id);

        // Validate that the worker is registered
        if !self.state.registered_workers.contains_key(&req.worker_id) {
            println!("Received heartbeat from unregistered worker: {}", req.worker_id);
            return Err(Status::not_found("Worker not registered"));
        }

        let heartbeat_info =
            HeartbeatInfo::new(req.worker_id.clone(), prost_types::Timestamp::from(std::time::SystemTime::now()));

        let response = match self
            .state
            .heartbeats
            .insert(req.worker_id.clone(), heartbeat_info)
        {
            Some(_) => {
                println!("Updated heartbeat for worker: {}", req.worker_id);
                HeartbeatResponse { action: HeartbeatAction::Updated as i32 }
            },
            None => {
                println!("Inserted new heartbeat for worker: {}", req.worker_id);
                HeartbeatResponse { action: HeartbeatAction::Inserted as i32 }
            },
        };

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::apis::registration::RegistrationInfo;

    #[tokio::test]
    async fn test_heartbeat() {
        let state = Arc::new(CoordinatorState::new());
        let heartbeat_service = HeartbeatService::new(Arc::clone(&state));

        // Register a worker
        let worker_id = "worker1".to_string();
        let hostname = "localhost".to_string();
        let registration_info = RegistrationInfo::new(hostname, prost_types::Timestamp::from(std::time::SystemTime::now()));
        state.registered_workers.insert(worker_id.clone(), registration_info);

        // Send a heartbeat
        let request = HeartbeatRequest { worker_id: worker_id.clone() };
        let response = heartbeat_service.heartbeat(Request::new(request)).await;

        // Verify the response and that the heartbeat was recorded
        let resp = response.unwrap().into_inner();
        assert_eq!(resp, HeartbeatResponse { action: HeartbeatAction::Inserted as i32 });
        assert!(state.heartbeats.contains_key(&worker_id));
    }

    #[tokio::test]
    async fn test_update_heartbeat_worker() {
        let state = Arc::new(CoordinatorState::new());
        let heartbeat_service = HeartbeatService::new(Arc::clone(&state));      

        // Register a worker
        let worker_id = "worker1".to_string();
        let hostname = "localhost".to_string();
        let registration_info = RegistrationInfo::new(hostname, prost_types::Timestamp::from(std::time::SystemTime::now()));
        state.registered_workers.insert(worker_id.clone(), registration_info);

        // Send a heartbeat
        let request1 = HeartbeatRequest { worker_id: worker_id.clone() };
        let request2 = HeartbeatRequest { worker_id: worker_id.clone() };
        #[allow(unused_variables)]
        let response1 =heartbeat_service.heartbeat(Request::new(request1)).await;
        let response2 = heartbeat_service.heartbeat(Request::new(request2)).await;
        
        // Verify the response and that the heartbeat was updated
        let resp = response2.unwrap().into_inner();
        assert_eq!(resp, HeartbeatResponse { action: HeartbeatAction::Updated as i32 });
        assert!(state.heartbeats.contains_key(&worker_id));
    }

    #[tokio::test]
    async fn test_heartbeat_unregistered_worker() {
        let state = Arc::new(CoordinatorState::new());
        let heartbeat_service = HeartbeatService::new(Arc::clone(&state));      

        // Send a heartbeat from an unregistered worker
        let request = HeartbeatRequest { worker_id: "unregistered_worker".to_string() };
        let response = heartbeat_service.heartbeat(Request::new(request)).await;

        // Verify that the response is an error and that no heartbeat was recorded
        assert!(response.is_err());
        assert!(!state.heartbeats.contains_key("unregistered_worker"));
    }
}