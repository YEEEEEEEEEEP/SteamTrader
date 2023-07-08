use reqwest::{Client, Error, Response};
use std::collections::HashMap;
use std::time::Duration;

pub async fn get_stats(client: reqwest::Client) -> Result<Response, Error> {
    client
        .get("https://www.valvesoftware.com/about/stats")
        .send()
        .await
}
enum Endpoint {
    GetStats,
    GetAppDetails,
}

impl Endpoint {
    const fn rate(self) -> &'static Rate {
        match self {
            Self::GetStats => &Rate {
                requests: 10000,
                duration: Duration::from_micros(1),
            },
            Self::GetAppDetails => &Rate {
                requests: 200,
                duration: Duration::from_secs(5 * 60),
            },
        }
    }
}

#[derive(Debug)]
pub struct Rate {
    pub requests: i32,
    pub duration: std::time::Duration,
}

#[cfg(test)]
mod tests;
