use proto::heartbeat::heartbeat_server::HeartbeatServer;
use proto::registration::registration_server::RegistrationServer;
use tonic::transport::Server;

mod apis;

use apis::heartbeat::HeartbeatService;
use apis::registration::RegistrationService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let registration_service = RegistrationService::new();
    let heartbeat_service = HeartbeatService::new();

    println!("Coordinator server listening on {}", addr);

    Server::builder()
        .add_service(RegistrationServer::new(registration_service))
        .add_service(HeartbeatServer::new(heartbeat_service))
        .serve(addr)
        .await?;

    Ok(())
}