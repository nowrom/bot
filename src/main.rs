use anyhow::Result;

use rombot::{
    discord::bot::start_discord, matrix::bot::start_matrix, telegram::bot::start_telegram,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv()?;

    // #[cfg(not(feature = "nodiscord"))]
    // tokio::spawn(async {
    start_discord().await.unwrap();
    // });
    // #[cfg(not(feature = "notelegram"))]
    // tokio::spawn(async {
    //     start_telegram().await;
    // });
    // #[cfg(not(feature = "nomatrix"))]
    // start_matrix().await?;

    Ok(())
}
