use dotenv::dotenv;
use proto::heartbeat::heartbeat_client::HeartbeatClient;
use proto::registration::registration_client::RegistrationClient;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = Config::from_env()?;
    let worker_id = config.worker_id;

    let mut registration_client = RegistrationClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(proto::registration::RegisterWorkerRequest {
        worker_id: worker_id.clone(),
        hostname: config.hostname,
    });

    let registered = registration_client.register(request).await?;
    println!("Registered worker: {:?}", registered);

    let heartbeat_handle = tokio::spawn(async move {
        let mut heartbeat_client = HeartbeatClient::connect("http://[::1]:50051")
            .await
            .expect("Failed to connect heartbeat client");

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!("Sending heartbeat...");

            let request = tonic::Request::new(proto::heartbeat::HeartbeatRequest {
                worker_id: worker_id.clone(),
            });

            match heartbeat_client.heartbeat(request).await {
                Ok(response) => println!("Heartbeat response: {:?}", response),
                Err(e) => eprintln!("Heartbeat failed: {}", e),
            }
        }
    });

    heartbeat_handle.await?;

    Ok(())
}
