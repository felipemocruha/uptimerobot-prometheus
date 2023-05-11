# uptimerobot-prometheus

The `uptimerobot-prometheus` project is a tool that collects reliability metrics from Uptime Robot and exposes them in Prometheus format through a /metrics server endpoint.

[Uptime Robot](Uptime Robot) is a monitoring service that checks if your websites, APIs, and servers are up and running. The Uptime Robot API allows developers to retrieve various information about their monitors, such as their status, response time, and more.

[Prometheus](https://prometheus.io/) is a monitoring system and time series database that collects and stores metrics from various sources. Prometheus provides a powerful query language to analyze and visualize metrics and can integrate with many alerting and dashboarding tools.

The `uptimerobot-prometheus` project bridges the gap between Uptime Robot and Prometheus by collecting metrics from the Uptime Robot API and exposing them in Prometheus format. This allows users to monitor their Uptime Robot monitors alongside other metrics in their Prometheus instance and create alerts and dashboards based on these metrics.

## Features
- Collects reliability metrics from Uptime Robot API
- Exposes metrics in Prometheus format through a `/metrics` server endpoint
- Can be configured to scrape multiple Uptime Robot accounts and monitors
- Can be run as a Docker container or as a standalone binary

## Exposed metrics

| Name | Description | Labels | Type |
|------|-------------|--------|------|
| `uptime_robot_collect_executions_total` | Total number of uptime robot scrapings for this process | is_success | `counter` |
| `uptime_robot_response_time_seconds` | Last recorded response_time from monitor in milliseconds | monitor, status | `histogram` |
| `uptime_robot_monitor_status` | The numeric status of monitor. See `status_as_str` for meaning | monitor, status_as_str | `gauge` |
| `uptime_robot_uptime_1d` | Uptime percentage in the last day | monitor | `gauge` |
| `uptime_robot_uptime_7d` | Uptime percentage in the last 7 days | monitor | `gauge` |
| `uptime_robot_uptime_30d` | Uptime percentage in the last 30 days | monitor | `gauge` |

## Getting Started

### Prerequisites

An Uptime Robot account and read-only API key. You can obtain an API key by [creating an Uptime Robot account](https://uptimerobot.com/signUp) and navigating to the [API settings page](API settings page).
(Optional) A Prometheus instance to scrape the metrics.

### Configuration

You can configure the application by setting the following environment variables, or creating a `.env` file at project root.

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `UPTIME_ROBOT_APIKEY` |  | An Uptime Robot read-only API key | `true` |
| `UPTIME_ROBOT_URL` | https://api.uptimerobot.com | Uptime Robot API URL | `false` |
| `SERVER_HOST` | 0.0.0.0:9882 | The host and port to bind the metrics server | `false` |
| `COLLECTOR_INTERVAL_SECONDS` | 60s | The number of seconds the collector will wait before polling Uptime Robot | `false` |

### Running

#### Development

```
cargo run
```

#### Docker

```
docker build -t <tag> -f Dockerfile .
docker run -ti -e UPTIME_ROBOT_APIKEY=<API key> -p 9882:9882 <tag>
```

## Known issues

The collector does not support pagination. Currently the API returns per page 50 monitors max, so this is the limit of monitors this app supports for now.

## Contributing

I'm not interested to maintain this project. The license is MIT, fork it and change whatever you want.
