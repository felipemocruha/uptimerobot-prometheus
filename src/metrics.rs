use prometheus::Registry;
use prometheus::{Counter, HistogramOpts, HistogramVec, Opts};
use std::error::Error;

pub struct Metrics {
    pub registry: Registry,
    pub counter: Counter,
    pub hist: HistogramVec,
}

impl Metrics {
    pub fn new() -> Result<Metrics, Box<dyn Error>> {
        let registry = Registry::new();

        let counter_opts = Opts::new("test_counter", "test counter help");
        let counter = Counter::with_opts(counter_opts)?;

        let hist = HistogramVec::new(
            HistogramOpts::new("response_time", "Response Times"),
            &["env"],
        )?;

        registry.register(Box::new(counter.clone()))?;
        registry.register(Box::new(hist.clone()))?;

        Ok(Metrics {
            registry: registry,
            counter: counter,
            hist: hist,
        })
    }
}
