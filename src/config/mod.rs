use color_eyre::{Result, eyre::WrapErr};
use serde::Deserialize;
use dotenv::dotenv;
use config::Config as ConfigLoader;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16, // Port should be u16, as it is for network ports
}

impl Config {
    pub fn from_env() -> Result<Config> {
        dotenv().ok(); // Load .env variables if available

        let mut c = ConfigLoader::new();
        c.merge(config::Environment::default())?; // Merge environment variables

        c.try_into()
            .context("loading configuration from environment") // Wrap errors with context
    }
}
