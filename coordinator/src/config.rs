use std::{env, net::SocketAddr};

#[allow(dead_code)]
pub struct Config {
    pub mapper_resources_dir: String,
    pub mapper_output_dir: String,
    pub addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mapper_resources_dir =
            env::var("MAPPER_RESOURCES_DIR").map_err(|e| format!("MAPPER_RESOURCES_DIR: {e}"))?;
        let mapper_output_dir =
            env::var("MAPPER_OUTPUT_DIR").map_err(|e| format!("MAPPER_OUTPUT_DIR: {e}"))?;
        let coordinator_port = env::var("COORDINATOR_PORT")
            .map_err(|e| format!("COORDINATOR_PORT: {e}"))?
            .parse::<u16>()?;
        let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], coordinator_port));

        Ok(Config {
            mapper_resources_dir,
            mapper_output_dir,
            addr,
        })
    }
}
