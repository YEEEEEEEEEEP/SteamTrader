use reqwest::{Client, Error, Response};
use std::collections::HashMap;
use std::time::Duration;

pub struct BackpackInterface {
    client: Client,
    rate_limits: HashMap<Endpoint, RateLimit>,
}

impl BackpackInterface {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            rate_limits: HashMap::new(),
        }
    }

    pub fn get_currencies() {}

    async fn rate_limit(mut limit: &mut RateLimit) -> Result<(), Error> {
        let filtered: Vec<Request> = limit
            .requests
            .clone()
            .into_iter()
            .filter(|r| r.time.elapsed() <= limit.rate.duration)
            .collect();
        limit.requests = filtered;
        if limit.requests.len() >= limit.rate.requests as usize {
            let time = limit.requests[0].time.elapsed();
            tokio::time::sleep(limit.rate.duration - time).await;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AppId {
    TF2 = 440,
    Dota2 = 570,
    CSGO = 730,
    Steam = 753,
    SteamVR = 250820,
    SteamVRHome = 848450,
    SteamVRMedia = 250840,
    SteamVRTools = 250860,
}

impl From<AppId> for u32 {
    fn from(appid: AppId) -> Self {
        appid as u32
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Endpoint {
    GetStats,
    GetAppDetails,
    GetItems,
}

#[derive(Debug)]
pub struct Rate {
    pub requests: i32,
    pub duration: std::time::Duration,
}

struct RateLimit {
    rate: Rate,
    requests: Vec<Request>,
}

#[derive(Debug, Clone)]
struct Request {
    endpoint: Endpoint,
    time: std::time::Instant,
}
