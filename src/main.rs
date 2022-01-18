use anyhow::Result;

use rombot::{
    discord::bot::start_discord, matrix::bot::start_matrix, telegram::bot::start_telegram,
    update_devices,
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
        start_telegram().await;
    });
    start_matrix().await.unwrap();

    Ok(())
}
