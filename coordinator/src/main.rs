use std::{time::Duration, sync::Arc, path};

use proto::heartbeat::heartbeat_server::HeartbeatServer;
use proto::registration::registration_server::RegistrationServer;
use tonic::transport::Server;

mod apis;
mod coordinator_state;
mod config;
mod mapper;

use config::Config;
use apis::heartbeat::HeartbeatService;
use apis::registration::RegistrationService;
use coordinator_state::CoordinatorState;
use mapper::coordinator_mapper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    coordinator_mapper::setup_mappers(path::Path::new(&config.mapper_resources_dir))?;

    let addr = "[::1]:50051".parse()?;

    let state = Arc::new(CoordinatorState::new());
    let registration_service = RegistrationService::new(Arc::clone(&state));
    let heartbeat_service = HeartbeatService::new(Arc::clone(&state));

    println!("Coordinator server listening on {}", addr);

    // Run the gRPC server in a separate task
    let server_handler = tokio::spawn( async move {
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
    });

    // Run the heartbeat monitoring in a separate task
    let hearbeat_handler = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;  

            // Iterate over all the heartbeats and check if any worker has failed to send a heartbeat in the last 10 seconds
            println!("Checking heartbeats...");
            state.process_heartbeat();
        }
    });

    // Wait for both tasks to complete (in practice, the server will run indefinitely)
    tokio::select! {
        res = server_handler => {
            if let Err(e) = res {
                eprintln!("Server error: {}", e);
            }
        },
        _ = hearbeat_handler => {
            println!("Hearbeat thread finished");
        }
    }

    Ok(())
}
