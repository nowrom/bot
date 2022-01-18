use anyhow::Result;
use fuse_rust::{Fuse, FuseProperty, Fuseable};
use futures::executor::block_on;
use lazy_static::lazy_static;
use rombot::{
    discord::bot::start_discord,
    matrix::bot::start_matrix,
    search,
    // telegram::bot::start_telegram,
    update_devices,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{rc::Rc, sync::Mutex};
use tokio::{
    task,
    time::{sleep, Duration},
};

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
