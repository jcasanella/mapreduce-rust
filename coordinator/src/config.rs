use std::{env, net::SocketAddr};

#[allow(dead_code)]
pub struct Config {
    pub mapper_resources_dir: String,
    pub mapper_output_dir: String,
    pub addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mapper_resources_dir = env::var("MAPPER_RESOURCES_DIR")?;
        let mapper_output_dir = env::var("MAPPER_OUTPUT_DIR")?;
        let coordinator_port = env::var("COORDINATOR_PORT")?.parse::<u16>()?;
        let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], coordinator_port));

        Ok(Config {
            mapper_resources_dir,
            mapper_output_dir,
            addr,
        })
    }
}
