use prometheus::Registry;
use prometheus::{CounterVec, GaugeVec, HistogramOpts, HistogramVec, IntGaugeVec, Opts};
use std::error::Error;

pub struct Metrics {
    pub registry: Registry,
    pub collector_executions: CounterVec,
    pub response_time: HistogramVec,
    pub monitor_status: IntGaugeVec,
    pub uptime_1d: GaugeVec,
    pub uptime_7d: GaugeVec,
    pub uptime_30d: GaugeVec,
}

impl Metrics {
    pub fn new() -> Result<Metrics, Box<dyn Error>> {
        let registry = Registry::new();

        let collector_executions = CounterVec::new(
            Opts::new(
                "uptime_robot_collect_executions_total",
                "Total number of uptime robot scrapings for this process",
            ),
            &["is_success"],
        )?;
        registry.register(Box::new(collector_executions.clone()))?;

        let response_time = HistogramVec::new(
            HistogramOpts::new(
                "uptime_robot_response_time_millis",
                "Last recorded response_time from monitor in milliseconds",
            ),
            &["monitor", "status"],
        )?;
        registry.register(Box::new(response_time.clone()))?;

        let monitor_status = IntGaugeVec::new(
            Opts::new(
                "uptime_robot_monitor_status",
                "The numeric status of monitor. See `status_as_str` for meaning",
            ),
            &["monitor", "status_as_str"],
        )?;
        registry.register(Box::new(monitor_status.clone()))?;

        let uptime_1d = GaugeVec::new(
            Opts::new(
                "uptime_robot_uptime_1d",
                "Uptime percentage in the last day",
            ),
            &["monitor"],
        )?;
        registry.register(Box::new(uptime_1d.clone()))?;

        let uptime_7d = GaugeVec::new(
            Opts::new(
                "uptime_robot_uptime_7d",
                "Uptime percentage in the last 7 days",
            ),
            &["monitor"],
        )?;
        registry.register(Box::new(uptime_7d.clone()))?;

        let uptime_30d = GaugeVec::new(
            Opts::new(
                "uptime_robot_uptime_30d",
                "Uptime percentage in the last 30 days",
            ),
            &["monitor"],
        )?;
        registry.register(Box::new(uptime_30d.clone()))?;

        Ok(Metrics {
            registry: registry,
            collector_executions: collector_executions,
            response_time: response_time,
            monitor_status: monitor_status,
            uptime_1d: uptime_1d,
            uptime_7d: uptime_7d,
            uptime_30d: uptime_30d,
        })
    }
}
