use std::sync::Arc;

use proto::heartbeat::heartbeat_server::HeartbeatServer;
use proto::registration::registration_server::RegistrationServer;
use tonic::transport::Server;

mod apis;
mod coordinator_state;

use apis::heartbeat::HeartbeatService;
use apis::registration::RegistrationService;
use coordinator_state::CoordinatorState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let state = Arc::new(CoordinatorState::new());
    let registration_service = RegistrationService::new(Arc::clone(&state));
    let heartbeat_service = HeartbeatService::new(Arc::clone(&state));

    println!("Coordinator server listening on {}", addr);

    Server::builder()
        .add_service(RegistrationServer::new(registration_service))
        .add_service(HeartbeatServer::new(heartbeat_service))
        .serve(addr)
        .await?;

    Ok(())
}
