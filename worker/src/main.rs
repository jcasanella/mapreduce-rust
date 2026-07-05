use dotenv::dotenv;

mod config;
mod heartbeat;
mod mapper;
mod registration;
use config::Config;
mod map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = Config::from_env()?; 
    registration::register_worker(&config).await?;

    let heartbeat_handler = tokio::spawn(heartbeat::run(config.clone()));
    let mapper_handler = tokio::spawn(mapper::run(config.clone()));

    tokio::select! {
        res = heartbeat_handler => {
            if let Err(e) = res {
                eprintln!("Heartbeat error: {}", e);
            }
        },
        res = mapper_handler => {
            if let Err(e) = res {
                eprintln!("Mapper error: {}", e);
            }
        }
    }

    Ok(())
}
