use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use proto::registration::registration_server::Registration;
use proto::registration::{RegisterWorkerRequest, RegisterWorkerResponse};
use tonic::{Request, Response, Status};

pub struct RegistrationService {
    workers: Arc<RwLock<HashMap<String, RegistrationInfo>>>,
}

impl RegistrationService {
    pub fn new() -> Self {
        Self {
            workers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Debug)]
pub struct RegistrationInfo {
    pub hostname: String,
    pub registered_at: prost_types::Timestamp,
}

impl RegistrationInfo {
    pub fn new(hostname: String) -> Self {
        RegistrationInfo {
            hostname,
            registered_at: prost_types::Timestamp::default(),
        }
    }
}

#[tonic::async_trait]
impl Registration for RegistrationService {
    async fn register(
        &self,
        request: Request<RegisterWorkerRequest>,
    ) -> Result<Response<RegisterWorkerResponse>, Status> {
        let RegisterWorkerRequest { worker_id, hostname } = request.into_inner();
        let registration = RegistrationInfo::new(hostname);
        let registered_at = registration.registered_at;
        println!("Registering worker with id: {} at {} - hostname: {}", worker_id, registered_at, registration.hostname);

        self.workers
            .write()
            .map_err(|e| Status::internal(e.to_string()))?
            .insert(worker_id, registration);

        let response = RegisterWorkerResponse {
            success: true,
            registered_at: Some(registered_at),
        };

        Ok(Response::new(response))
    }
}

