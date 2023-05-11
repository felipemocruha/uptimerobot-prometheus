use crate::metrics::Metrics;
use crate::uptime_robot::Client;
use std::error::Error;
use std::sync::Arc;

pub struct Collector {
    pub client: Arc<Client>,
    pub metrics: Arc<Metrics>,
}

impl Collector {
    pub fn new(client: Arc<Client>, metrics: Arc<Metrics>) -> Collector {
        Collector {
            client: client,
            metrics: metrics,
        }
    }

    pub async fn collect(&self) -> Result<(), Box<dyn Error>> {
        let monitors = self.client.get_monitors().await?;
        println!("{:?}", monitors);

        Ok(())
    }
}
