use anyhow::Result;
use log::error;
use std::{env, time::Duration};
use telexide_fork::{
    api::types::{SendMessage, UpdateType},
    prelude::*,
};
use tokio::time::sleep;

#[command(description = "Find a rom for your needs")]
async fn rom(context: Context, message: Message) -> CommandResult {
    context
        .api
        .send_message(SendMessage::new(message.chat.get_id(), "pong"))
        .await?;
    Ok(())
}

pub async fn start_telegram() -> Result<()> {
    let token = env::var("TELEGRAM_TOKEN").expect("no token environment variable set");
    let bot_name = env::var("TELEGRAM_NAME").expect("Provide the bot name via BOT_NAME env var");

    let client = ClientBuilder::new()
        .set_token(&token)
        .set_framework(create_framework!(&bot_name, rom))
        .set_allowed_updates(vec![UpdateType::CallbackQuery, UpdateType::Message])
        .build();

    loop {
        log::info!("Starting start loop of bot...");
        let ret = client.start().await;
        match ret {
            Err(err) => {
                error!("ApiResponse {}\nWaiting a minute and retrying...", err);
                sleep(Duration::from_secs(60)).await;
            }
            Ok(()) => {
                error!("Exiting from main loop without an error, but this should never happen!");
                break;
            }
        }
    }
    Ok(())
}
