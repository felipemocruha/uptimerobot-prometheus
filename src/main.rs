mod collector;
mod config;
mod metrics;
mod server;
mod uptime_robot;

use log::error;
use std::error::Error;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use collector::Collector;
use config::Config;
use metrics::Metrics;
use server::{Server, ServerState};
use uptime_robot::{Client, ClientOpts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;

    let opts = ClientOpts {
        apikey: config.uptime_robot_apikey,
        base_url: config.uptime_robot_base_url,
    };
    let client = Arc::new(Client::new(opts));
    let metrics = Arc::new(Metrics::new()?);
    let state = Arc::new(ServerState {
        metrics: metrics.clone(),
    });
    let collector = Collector::new(client.clone(), metrics.clone());

    tokio::spawn(async move {
        loop {
            collector
                .collect()
                .await
                .map_err(|err| error!("collect data: {}", err))
                .ok();

            sleep(Duration::from_secs(config.collector_interval)).await;
        }
    });

    let server = Server::new(config.server_host, state);
    server.start().await?;

    Ok(())
}
