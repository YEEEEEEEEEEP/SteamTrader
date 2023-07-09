use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use steam_interface::{AppId, SteamInterface};

#[tokio::main]
async fn main() {
    let mut items: HashSet<Item> = HashSet::new();
    let cookies = std::fs::read_to_string("cookies.txt").unwrap();
    let jar = std::sync::Arc::new(reqwest::cookie::Jar::default());
    jar.add_cookie_str(
        &cookies,
        &reqwest::Url::parse("https://steamcommunity.com").unwrap(),
    );
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar)
        .build()
        .unwrap();
    let mut interface = SteamInterface::new(client);
    let total = interface
        .get_items(AppId::TF2, 1, 0)
        .await
        .unwrap()
        .json::<Items>()
        .await
        .unwrap()
        .total_count;
    for i in (0..total).step_by(100) {
        let result = interface
            .get_items(AppId::TF2, 100, i)
            .await
            .unwrap()
            .json::<Items>()
            .await
            .unwrap();
        println!("{:?}", result);
        items.extend(result.results);
        let file = File::create("items.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &items).unwrap();
        writer.flush().unwrap();
    }
    let file = File::create("items.json").unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &items).unwrap();
    writer.flush().unwrap();
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
struct Items {
    pub success: bool,
    pub start: u32,
    pub pagesize: u32,
    pub total_count: u32,
    pub results: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
struct SearchData {
    pub query: String,
    pub search_descriptions: bool,
    pub total_count: u32,
    pub pagesize: u32,
    pub prefix: String,
    pub class_prefix: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
struct Item {
    name: String,
    sell_listings: u32,
    sell_price: u32,
    sell_price_text: String,
    asset_description: AssetDescription,
    // app_icon: String,
    // app_name: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
struct AssetDescription {
    pub appid: u32,
    pub classid: String,
    pub instanceid: String,
    // pub icon_url: String,
    pub tradable: u32,
    #[serde(rename = "type")]
    pub type_: String,
    pub market_name: String,
    pub market_hash_name: String,
    pub commodity: u32,
    pub name_color: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
struct Stats {
    pub users_online: String,
    pub users_ingame: String,
}
