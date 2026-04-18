use std::env;

pub struct Config {
    pub mapper_resources_dir: String,
    pub mapper_output_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let mapper_resources_dir = env::var("MAPPER_RESOURCES_DIR")?;
        let mapper_output_dir = env::var("MAPPER_OUTPUT_DIR")?;

        Ok(Config {
            mapper_resources_dir,
            mapper_output_dir,
        })
    }
}