use reqwest::{Client as HttpClient, Error as ReqwestError};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

pub struct ClientOpts {
    pub apikey: String,
    pub base_url: String,
}

pub struct Client {
    http_client: HttpClient,
    pub apikey: String,
    pub base_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetMonitorsResponse {
    pub monitors: Vec<Monitors>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Monitors {
    pub id: u64,
    pub friendly_name: String,
    pub url: String,
    pub status: MonitorStatus,
    pub response_times: Vec<ResponseTime>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseTime {
    pub value: u16,
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum MonitorStatus {
    Paused = 0,
    NotCheckedYet = 1,
    Up = 2,
    SeemsDown = 8,
    Down = 9,
}

impl Client {
    pub fn new(opts: ClientOpts) -> Client {
        Client {
            http_client: reqwest::Client::new(),
            apikey: opts.apikey,
            base_url: opts.base_url,
        }
    }

    pub async fn get_monitors(&self) -> Result<GetMonitorsResponse, ReqwestError> {
        let url = format!("{}/v2/getMonitors", self.base_url);

        let form = [
            ("api_key", &self.apikey),
            ("format", &String::from("json")),
            ("response_times", &String::from("1")), // ask for response time in response
            ("response_times_limit", &String::from("1")), // only get the last
        ];

        let response = self
            .http_client
            .post(&url)
            .header("Cache-Control", "no-cache")
            .form(&form)
            .send()
            .await?;

        let data: GetMonitorsResponse = response.json().await?;
        Ok(data)
    }
}
