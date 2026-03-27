use std::sync::Arc;

use crate::coordinator_state::CoordinatorState;
use proto::registration::registration_server::Registration;
use proto::registration::{RegisterWorkerRequest, RegisterWorkerResponse};
use tonic::{Request, Response, Status};

pub struct RegistrationService {
    state: Arc<CoordinatorState>,
}

impl RegistrationService {
    pub fn new(state: Arc<CoordinatorState>) -> Self {
        Self { state }
    }
}

#[derive(Debug)]
pub struct RegistrationInfo {
    pub hostname: String,
    pub registered_at: prost_types::Timestamp,
}

impl RegistrationInfo {
    pub fn new(hostname: String, registered_at: prost_types::Timestamp) -> Self {
        RegistrationInfo {
            hostname,
            registered_at,
        }
    }
}

#[tonic::async_trait]
impl Registration for RegistrationService {
    async fn register(
        &self,
        request: Request<RegisterWorkerRequest>,
    ) -> Result<Response<RegisterWorkerResponse>, Status> {
        let RegisterWorkerRequest {
            worker_id,
            hostname,
        } = request.into_inner();
        let registration = RegistrationInfo::new(hostname, prost_types::Timestamp::from(std::time::SystemTime::now()));
        let registered_at = registration.registered_at;
        println!(
            "Registering worker with id: {} at {} - hostname: {}",
            worker_id, registered_at, registration.hostname
        );

        if self.state.registered_workers.contains_key(&worker_id) {
            println!("Worker with id {} is already registered, can not register again", worker_id);
            return Err(Status::already_exists("Worker is already registered"));
        }

        self.state
            .registered_workers
            .insert(worker_id, registration);

        let response = RegisterWorkerResponse {
            success: true,
            registered_at: Some(registered_at),
        };

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_worker() {
        let state = Arc::new(CoordinatorState::new());
        let registration_service = RegistrationService::new(state.clone());

        let worker_id = "worker-123".to_string();
        let hostname = "localhost".to_string();
        let request = RegisterWorkerRequest { worker_id: worker_id.clone(), hostname: hostname.clone() };
        let response = registration_service.register(Request::new(request)).await;

        assert!(response.is_ok());
        assert!(state.registered_workers.contains_key(&worker_id));
    }

    #[tokio::test]
    async fn test_register_worker_multiple_times() {
        let state = Arc::new(CoordinatorState::new());
        let registration_service = RegistrationService::new(state.clone()); 

        let worker_id = "worker-123".to_string();
        let hostname1 = "localhost".to_string();
        let hostname2 = "localhost2".to_string();
        let request1 = RegisterWorkerRequest { worker_id: worker_id.clone(), hostname: hostname1.clone() };
        let request2 = RegisterWorkerRequest { worker_id: worker_id.clone(), hostname: hostname2.clone() };
        let response1 = registration_service.register(Request::new(request1)).await;
        let response2 = registration_service.register(Request::new(request2)).await;

        assert!(response1.is_ok());
        assert!(response2.is_err());
        assert_eq!(response2.unwrap_err().code(), tonic::Code::AlreadyExists);
        assert!(state.registered_workers.contains_key(&worker_id));
    }

    #[tokio::test]
    async fn test_register_multiple_workers() {
        let state = Arc::new(CoordinatorState::new());
        let registration_service = RegistrationService::new(state.clone());

        let worker_id1 = "worker-123".to_string();
        let worker_id2 = "worker-456".to_string();
        let hostname1 = "localhost".to_string();
        let hostname2 = "localhost2".to_string();
        let request1 = RegisterWorkerRequest { worker_id: worker_id1.clone(), hostname: hostname1.clone() };
        let request2 = RegisterWorkerRequest { worker_id: worker_id2.clone(), hostname: hostname2.clone() };
        let response1 = registration_service.register(Request::new(request1)).await;
        let response2 = registration_service.register(Request::new(request2)).await;

        assert!(response1.is_ok());
        assert!(response2.is_ok());
        assert!(state.registered_workers.contains_key(&worker_id1));
        assert!(state.registered_workers.contains_key(&worker_id2));
    }
}