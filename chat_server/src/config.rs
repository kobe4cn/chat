use std::fs::File;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    pub sk: String,
    pub pk: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db_url: String,
}

impl AppConfig {
    pub fn try_load() -> Result<Self> {
        // read from ./app.yml, or /etc/config/app.yml, or from env CHAT_CONFIG
        let ret = match (
            File::open("chat_server/app.yaml"),
            File::open("/etc/config/app.yaml"),
            std::env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("No config file found"),
        };

        Ok(ret?)
    }
}
