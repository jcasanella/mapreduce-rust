use dotenv::dotenv;
use proto::heartbeat::heartbeat_client::HeartbeatClient;
use proto::mapper::mapper_client::MapperClient;
use proto::registration::registration_client::RegistrationClient;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = Config::from_env()?;
    let worker_id = config.worker_id;
    let coordinator_addr = config.coordinator_addr;

    let mut registration_client =
        RegistrationClient::connect(coordinator_addr.clone()).await?;

    let request = tonic::Request::new(proto::registration::RegisterWorkerRequest {
        worker_id: worker_id.clone(),
        hostname: config.hostname,
    });

    let registered = registration_client.register(request).await?;
    println!("Registered worker: {:?}", registered);

    let heartbeat_handle = tokio::spawn(async move {
        let mut heartbeat_client =
            HeartbeatClient::connect(coordinator_addr.clone())
                .await
                .expect("Failed to connect heartbeat client");

        let has_task = false;
        let mut mapper_client =
            MapperClient::connect(coordinator_addr).await.expect("Failed to connect mapper client");

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!("Sending heartbeat...");

            let request = tonic::Request::new(proto::heartbeat::HeartbeatRequest {
                worker_id: worker_id.clone(),
            });

            if !has_task {
                // TODO - check what returns if can assign a task or not
                mapper_client.get_new_task(tonic::Request::new(())).await.expect("Failed to get new task");
            }

            match heartbeat_client.heartbeat(request).await {
                Ok(response) => println!("Heartbeat response: {:?}", response),
                Err(e) => eprintln!("Heartbeat failed: {}", e),
            }
        }
    });

    heartbeat_handle.await?;

    Ok(())
}
