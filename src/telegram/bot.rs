use tbot::{prelude::*, Bot};

use crate::{codename, format_device, search};

pub async fn start_telegram() {
    let mut bot = Bot::from_env("TELEGRAM_TOKEN").event_loop();
    bot.text(|context| async move {
        let body = &context.text.value;
        if body.contains(".rom") {
            let mut iter = body.split(' ');
            iter.next();
            let phone = iter.collect::<Vec<&str>>().join(" ");
            let text = if let Some(device) = codename(phone.clone()) {
                format_device(device, vec![])
            } else if let Some((device, alternatives)) = search(phone) {
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

    bot.polling().start().await.unwrap();
}
