use proto::registration::registration_client::RegistrationClient;

pub async fn register_worker(
    config: &crate::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut registration_client =
        RegistrationClient::connect(config.coordinator_addr.clone()).await?;

    let request = tonic::Request::new(proto::registration::RegisterWorkerRequest {
        worker_id: config.worker_id.clone(),
        hostname: config.hostname.clone(),
    });

    let registered = registration_client.register(request).await?;
    println!("Registered worker: {:?}", registered);

    Ok(())
}
