mod collector;
mod config;
mod metrics;
mod server;
mod uptime_robot;

use log::info;
use std::error::Error;
use std::sync::Arc;
extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};

use collector::Collector;
use config::Config;
use metrics::Metrics;
use server::{Context, Server};
use uptime_robot::{Client, ClientOpts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().json().init();

    let config = Config::new()?;
    let opts = ClientOpts {
        apikey: config.uptime_robot_apikey,
        base_url: config.uptime_robot_base_url,
    };
    let client = Arc::new(Client::new(opts));
    let metrics = Arc::new(Metrics::new()?);
    let ctx = Arc::new(Context {
        metrics: metrics.clone(),
    });
    let collector = Collector::new(client.clone(), metrics.clone(), config.collector_interval);

    tokio::spawn(async move {
        info!("collector started");
        collector.start().await;
    });

    tokio::spawn(async move {
        info!("server started");
        let server = Server::new(config.server_host, ctx);
        server.start().await.unwrap();
    });

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("failed to set Ctrl-C handler");

    info!("app started");
    while running.load(Ordering::SeqCst) {}

    Ok(())
}
