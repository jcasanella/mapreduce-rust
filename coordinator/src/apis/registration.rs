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
