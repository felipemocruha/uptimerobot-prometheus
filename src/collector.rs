use log::{error, info};
use std::error::Error;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use crate::metrics::Metrics;
use crate::uptime_robot::Client;

pub struct Collector {
    pub client: Arc<Client>,
    pub metrics: Arc<Metrics>,
    pub interval: u64,
}

impl Collector {
    pub fn new(client: Arc<Client>, metrics: Arc<Metrics>, interval: u64) -> Collector {
        Collector {
            client,
            metrics,
            interval,
        }
    }

    pub async fn collect(&self) -> Result<(), Box<dyn Error>> {
        let monitors = self.client.get_monitors().await?.monitors;

        for m in monitors.into_iter() {
            let duration = if m.response_times.is_empty() {
                0.0
            } else {
                m.response_times[0].value
            };

            let uptimes: Vec<&str> = m.custom_uptime_ratio.split('-').collect::<Vec<_>>();

            let (uptime_1d, uptime_7d, uptime_30d) = match uptimes[..] {
                [uptime_1d, uptime_7d, uptime_30d] => (uptime_1d, uptime_7d, uptime_30d),
                _ => ("0.0", "0.0", "0.0"),
            };

            self.metrics
                .response_time
                .with_label_values(&[&m.friendly_name, &m.status.to_string()])
                .observe(duration / 1000.0);

            self.metrics
                .monitor_status
                .with_label_values(&[&m.friendly_name, &m.status.to_string()])
                .set(m.status as i64);

            self.metrics
                .uptime_1d
                .with_label_values(&[&m.friendly_name])
                .set(uptime_1d.parse::<f64>().unwrap());

            self.metrics
                .uptime_7d
                .with_label_values(&[&m.friendly_name])
                .set(uptime_7d.parse::<f64>().unwrap());

            self.metrics
                .uptime_30d
                .with_label_values(&[&m.friendly_name])
                .set(uptime_30d.parse::<f64>().unwrap());
        }

        Ok(())
    }

    pub async fn start(&self) {
        loop {
            info!("starting a collector execution");
            let mut is_success = "true";

            self.collect()
                .await
                .map_err(|err| {
                    is_success = "false";
                    error!("collect data: {}", err);
                })
                .ok();

            self.metrics
                .collector_executions
                .with_label_values(&[is_success])
                .inc();

            info!(
                "collector execution finished, sleeping for {}s",
                self.interval
            );
            sleep(Duration::from_secs(self.interval)).await;
        }
    }
}
