use dotenv::dotenv;
use envconfig::{Envconfig, Error as EnvconfigError};

#[derive(Debug, Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "UPTIME_ROBOT_APIKEY")]
    pub uptime_robot_apikey: String,

    #[envconfig(from = "UPTIME_ROBOT_URL", default = "https://api.uptimerobot.com")]
    pub uptime_robot_base_url: String,

    #[envconfig(from = "SERVER_HOST", default = "0.0.0.0:9882")]
    pub server_host: String,

    #[envconfig(from = "COLLECTOR_INTERVAL_SECONDS", default = "60")]
    pub collector_interval: u64,
}

impl Config {
    pub fn new() -> Result<Config, EnvconfigError> {
        dotenv().ok();
        let config = Config::init_from_env()?;

        Ok(config)
    }
}
