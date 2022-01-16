use anyhow::Result;
use rombot::{
    discord::bot::start_discord, matrix::bot::start_matrix, telegram::bot::start_telegram,
};
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv()?;

    tokio::spawn(async {
        start_discord().await.unwrap();
    });
    tokio::spawn(async {
        start_matrix().await;
    });
    // tokio::spawn(async {
    //     start_telegram().await.unwrap();
    // });
    start_telegram().await.unwrap();
    loop {}
}
