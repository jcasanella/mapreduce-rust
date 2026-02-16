use proto::registration::registration_server::Registration;
use proto::registration::{RegisterWorkerRequest, RegisterWorkerResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyRegistration {}

#[tonic::async_trait]
impl Registration for MyRegistration {
     async fn register(
         &self,
         request: Request<RegisterWorkerRequest>,
     ) -> Result<Response<RegisterWorkerResponse>, Status> {
         let req = request.into_inner();
         println!("Registering worker: {} at {}", req.worker_id, req.hostname);

         let response = RegisterWorkerResponse {
             success: true,
             registered_at: Some(prost_types::Timestamp::default()),
         };

         Ok(Response::new(response))
     }
}
