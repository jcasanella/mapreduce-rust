use proto::registration::registration_client::RegistrationClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut worker = RegistrationClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(proto::registration::RegisterWorkerRequest {
        worker_id: "worker-1".to_string(),
        hostname: "worker1.local".to_string(),
    });

    let registered = worker.register(request).await?;
    println!("Registered worker: {:?}", registered);

    Ok(())
}
