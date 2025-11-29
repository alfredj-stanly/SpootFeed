use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env::var;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env file if exists
        dotenv().ok();

        Ok(Config {
            host: var("HOST").unwrap(),
            port: var("PORT")
                .unwrap()
                .parse()
                .context("PORT must be a valid number")?,
        })
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
