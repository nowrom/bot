use std::env;

use tbot::{prelude::*, Bot};

use crate::{codename, format_device, search};

pub async fn start_telegram() {
    let mut bot = Bot::from_env("TELEGRAM_TOKEN").event_loop();
    bot.username(env::var("TELEGRAM_NAME").unwrap());

    bot.text(|context| async move {
        let body = &context.text.value;
        if body.contains(".rom") {
            let mut iter = body.split(' ');
            iter.next();
            let phone = iter.collect::<Vec<&str>>().join(" ");
            let text = if let Some(device) = codename(phone.clone()).await {
                format_device(device, vec![])
            } else if let Some((device, alternatives)) = search(phone).await {
                format_device(device, alternatives)
            } else {
                "Phone not found".to_owned()
            };

            let call_result = context.send_message(&text).call().await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });
    bot.command_with_description("rom", "Find a rom for your phone", |e| async move {
        e.send_message(
            "please use .rom <phone> for this command, make sure i have read messages permissions",
        )
        .call()
        .await
        .unwrap();
    });

    bot.polling().start().await.unwrap();
}
