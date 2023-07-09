use reqwest::{Client, Error, Response};
use std::collections::HashMap;
use std::time::Duration;

pub mod structs;
use structs::{AppId, Endpoint, Rate, RateLimit, Request};

pub struct SteamInterface {
    client: Client,
    rate_limits: HashMap<Endpoint, RateLimit>,
}
impl SteamInterface {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            rate_limits: HashMap::new(),
        }
    }

    pub async fn get_stats(&mut self) -> Result<Response, Error> {
        Self::rate_limit(
            self.rate_limits
                .entry(Endpoint::GetStats)
                .or_insert(RateLimit {
                    rate: Rate {
                        requests: 1000,
                        duration: Duration::from_secs(1),
                    },
                    requests: Vec::new(),
                }),
        )
        .await?;
        self.rate_limits
            .get_mut(&Endpoint::GetStats)
            .unwrap()
            .requests
            .push(Request {
                endpoint: Endpoint::GetStats,
                time: std::time::Instant::now(),
            });
        self.client
            .get("https://www.valvesoftware.com/about/stats")
            .send()
            .await
    }

    pub async fn get_items(
        &mut self,
        appid: AppId,
        count: u32,
        start: u32,
    ) -> Result<Response, Error> {
        Self::rate_limit(
            self.rate_limits
                .entry(Endpoint::GetItems)
                .or_insert(RateLimit {
                    rate: Rate {
                        requests: 1,
                        duration: Duration::from_secs(1),
                    },
                    requests: Vec::new(),
                }),
        )
        .await?;
        self.rate_limits
            .get_mut(&Endpoint::GetItems)
            .unwrap()
            .requests
            .push(Request {
                endpoint: Endpoint::GetItems,
                time: std::time::Instant::now(),
            });
        self.client
            .get("https://steamcommunity.com//market/search/render/")
            .query(&[
                ("search_descriptions", 0),
                ("appid", appid.into()),
                ("currency", 3),
                ("count", count),
                ("start", start),
                ("norender", 1),
            ])
            .query(&[("sort_column", "name"), ("sort_dir", "desc")])
            .send()
            .await
    }

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

#[cfg(test)]
mod tests;
