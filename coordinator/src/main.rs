use std::{path, sync::Arc};

mod apis;
mod config;
mod coordinator_state;
mod heartbeat;
mod mapper;
mod server;

use config::Config;
use coordinator_state::CoordinatorState;
use mapper::coordinator_mapper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    coordinator_mapper::setup_mappers(path::Path::new(&config.mapper_resources_dir))?;

    let state = Arc::new(CoordinatorState::new());

    // Run the gRPC server in a separate task
    let server_handler = tokio::spawn(server::run(config.addr, Arc::clone(&state)));

    // Run the heartbeat monitoring in a separate task
    let heartbeat_handler = tokio::spawn(heartbeat::run(Arc::clone(&state)));

    // Wait for both tasks to complete (in practice, the server will run indefinitely)
    tokio::select! {
        res = server_handler => {
            if let Err(e) = res {
                eprintln!("Server error: {}", e);
            }
        },
        _ = heartbeat_handler => {
            println!("Hearbeat thread finished");
        }
    }

    Ok(())
}
