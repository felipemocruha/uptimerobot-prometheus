use crate::metrics::Metrics;
use axum::{extract::State, routing::get, Router};
use prometheus::{Encoder, TextEncoder};
use std::error::Error;
use std::sync::Arc;

pub struct Server {
    host: String,
    state: Arc<ServerState>,
}

pub struct ServerState {
    pub metrics: Arc<Metrics>,
}

impl Server {
    pub fn new(host: String, state: Arc<ServerState>) -> Server {
        let server = Server {
            host: host,
            state: state,
        };

        server
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let router = Router::new()
            .route("/metrics", get(prometheus))
            .with_state(self.state.clone());

        axum::Server::bind(&self.host.parse()?)
            .serve(router.into_make_service())
            .await?;

        Ok(())
    }
}

pub async fn prometheus(State(state): State<Arc<ServerState>>) -> String {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = state.metrics.registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}
