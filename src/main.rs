use serde::Deserialize;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client = reqwest::Client::new();
    let response: Stats = steam_interface::get_stats(client)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("{:?}", response);
}

#[derive(Deserialize, Debug)]
struct Stats {
    pub users_online: String,
    pub users_ingame: String,
}
