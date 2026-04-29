use std::env;

pub struct Config {
    pub worker_id: String,
    pub hostname: String,
    pub coordinator_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let worker_id = env::var("WORKER_ID").map_err(|e| format!("WORKER_ID: {e}"))?;
        let coordinator_port = env::var("COORDINATOR_PORT")
            .map_err(|e| format!("COORDINATOR_PORT: {e}"))?
            .parse::<u16>()
            .map_err(|e| format!("COORDINATOR_PORT must be a valid port number: {e}"))?;
        let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "worker.local".to_string());

        Ok(Config {
            worker_id,
            hostname,
            coordinator_addr: format!("http://[::1]:{coordinator_port}"),
        })
    }
}
