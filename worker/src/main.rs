use proto::registration::registration_client::RegistrationClient;
use dotenv::dotenv;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = Config::from_env()?;
    let mut worker = RegistrationClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(proto::registration::RegisterWorkerRequest {
        worker_id: config.worker_id,
        hostname: config.hostname,
    });

    let registered = worker.register(request).await?;
    println!("Registered worker: {:?}", registered);

    Ok(())
}
