use crate::metrics::Metrics;
use axum::{extract::State, routing::get, Router};
use prometheus::{Encoder, TextEncoder};
use std::error::Error;
use std::sync::Arc;

pub struct Server {
    host: String,
    ctx: Arc<Context>,
}

pub struct Context {
    pub metrics: Arc<Metrics>,
}

impl Server {
    pub fn new(host: String, ctx: Arc<Context>) -> Server {
        Server { host, ctx }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let router = Router::new()
            .route("/metrics", get(prometheus))
            .with_state(self.ctx.clone());

        axum::Server::bind(&self.host.parse()?)
            .serve(router.into_make_service())
            .await?;

        Ok(())
    }
}

pub async fn prometheus(State(ctx): State<Arc<Context>>) -> String {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = ctx.metrics.registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}
