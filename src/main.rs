mod collector;
mod config;
mod metrics;
mod server;
mod uptime_robot;

use std::error::Error;
use std::sync::Arc;

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
    let collector = Collector::new(client.clone(), metrics.clone(), config.collector_interval);

    tokio::spawn(async move {
        collector.start().await;
    });

    let server = Server::new(config.server_host, state);
    server.start().await?;

    Ok(())
}
