use proto::registration::registration_server::RegistrationServer;
use tonic::transport::Server;

mod apis;

use apis::registration::MyRegistration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let registration = MyRegistration::default();

    Server::builder()
        .add_service(RegistrationServer::new(registration))
        .serve(addr)
        .await?;

    Ok(())
}