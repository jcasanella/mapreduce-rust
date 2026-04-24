use std::{net::SocketAddr, sync::Arc};

use proto::heartbeat::heartbeat_server::HeartbeatServer;
use proto::registration::registration_server::RegistrationServer;
use tonic::transport::Server;

use crate::apis::heartbeat::HeartbeatService;
use crate::apis::registration::RegistrationService;
use crate::coordinator_state::CoordinatorState;

pub async fn run(addr: SocketAddr, state: Arc<CoordinatorState>) -> Result<(), tonic::transport::Error> {
    let registration_service = RegistrationService::new(Arc::clone(&state));
    let heartbeat_service = HeartbeatService::new(Arc::clone(&state));

    println!("Coordinator server listening on {}", addr);
    
    Server::builder()
        .add_service(RegistrationServer::new(registration_service))
        .add_service(HeartbeatServer::new(heartbeat_service))
        .serve_with_shutdown(addr, async {
            // Wait for a shutdown signal (e.g., Ctrl+C)
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for shutdown signal");
            println!("Shutdown signal received. Shutting down server...");
        })
        .await
}