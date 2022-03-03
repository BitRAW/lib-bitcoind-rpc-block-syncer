use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(rename(deserialize = "bitcoind-rpc"), rename(deserialize = "rpc"))]
    pub bitcoindrpc: BitcoindRpc,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BitcoindRpc {
    pub url: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
    #[serde(
        rename(deserialize = "polling-frequency-millis"),
        rename(deserialize = "polling")
    )]
    pub polling_frequency_millis: u64,
    #[serde(
        rename(deserialize = "con-revival-interval-millis"),
        rename(deserialize = "revival")
    )]
    pub revival_interval_millis: u64,
}

impl Settings {
    pub fn new(config_file_path: &str) -> Self {
        let err_msg = format!(
            "Failed to load settings! Create a file {} according to the template ./sample-config.toml OR pass the settings through environment variables as described in the README",
            config_file_path
        );

        match Self::load_settings(config_file_path) {
            Err(e) => panic!("{} Error Message: {}", err_msg, e),
            Ok(result) => result,
        }
    }

    fn load_settings(config_file_path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(config_file_path).required(false))
            .add_source(Environment::with_prefix("BTC").separator("_"))
            .build()?;

        s.try_deserialize()
    }
}
