use proto::heartbeat::heartbeat_client::HeartbeatClient;
// use proto::mapper::{GetNewTaskRequest, GetNewTaskResponse, mapper_client::MapperClient};

use crate::config;

pub async fn run(config: config::Config) -> Result<(), Box<dyn std::error::Error + Send>> {
    let mut heartbeat_client = HeartbeatClient::connect(config.coordinator_addr.clone())
        .await
        .expect("Failed to connect heartbeat client");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        println!("Sending heartbeat...");

        let request = tonic::Request::new(proto::heartbeat::HeartbeatRequest {
            worker_id: config.worker_id.clone(),
        });

        match heartbeat_client.heartbeat(request).await {
            Ok(response) => println!("Heartbeat response: {:?}", response),
            Err(e) => eprintln!("Heartbeat failed: {}", e),
        }
    }
}
