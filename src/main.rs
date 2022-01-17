use anyhow::Result;
use futures::executor::block_on;
use lazy_static::lazy_static;
use rombot::{
    discord::bot::start_discord, matrix::bot::start_matrix, telegram::bot::start_telegram,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{rc::Rc, sync::Mutex};
use tokio::{
    task,
    time::{sleep, Duration},
};
#[derive(Deserialize, Serialize, Debug)]
struct RomDevice {
    id: String,
}

fn default_resource() -> String {
    "Unknown".to_string()
}

#[derive(Deserialize, Serialize, Debug)]
struct Device {
    #[serde(default = "default_resource")]
    name: String,
    #[serde(default = "default_resource")]
    codename: String,
    #[serde(default = "default_resource")]
    brand: String,
    roms: Vec<RomDevice>,
}

lazy_static! {
    static ref DATA: Mutex<Vec<Device>> = Mutex::new(vec![]);
}

async fn update_devices() {
    let text = reqwest::Client::new()
        .get("https://nowrom.deno.dev")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut data: Vec<Device> = serde_json::from_str(&text).unwrap();
    let mut devices = DATA.lock().unwrap();
    devices.clear();
    devices.append(&mut data);
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv()?;
    update_devices().await;

    tokio::spawn(async {
        start_discord().await.unwrap();
    });
    tokio::spawn(async {
        block_on(async {
            start_matrix().await.unwrap();
        });
    });
    // tokio::spawn(async {
    //     block_on(async {
    start_telegram().await.unwrap();
    // });
    // });
    // loop {
    //     sleep(Duration::from_secs(60 * 60 * 24 * 7)).await;
    // }
    Ok(())
}
