use std::env;

pub struct Config {
    pub worker_id: String,
    pub hostname: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let worker_id = env::var("WORKER_ID")?;
        let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "worker.local".to_string());
        Ok(Config { worker_id, hostname })
    }
}