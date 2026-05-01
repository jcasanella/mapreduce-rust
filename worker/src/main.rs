use dotenv::dotenv;

mod config;
mod heartbeat;
mod registration;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = Config::from_env()?;
    registration::register_worker(&config).await?;
    heartbeat::run(config).await?;

    Ok(())
}
