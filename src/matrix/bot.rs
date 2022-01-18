use std::env;
use std::time::Duration;

use anyhow::Result;
use matrix_sdk::ruma::events;
use matrix_sdk::ruma::events::room::member::MemberEventContent;
use matrix_sdk::ruma::events::room::message::{
    MessageEventContent, MessageType, TextMessageEventContent,
};
use matrix_sdk::ruma::events::{AnyMessageEventContent, SyncMessageEvent};
use matrix_sdk::ruma::events::{RoomAccountDataEventContent, StrippedStateEvent};
use matrix_sdk::{room::Room, Client};
use matrix_sdk::{ClientConfig, SyncSettings};
use reqwest::Url;
use tokio::time::sleep;

use crate::{codename, format_device, search};

async fn on_room_message(event: SyncMessageEvent<MessageEventContent>, room: Room) {
    if let Room::Joined(room) = room {
        let msg_body = if let SyncMessageEvent {
            content:
                MessageEventContent {
                    msgtype: MessageType::Text(TextMessageEventContent { body: msg_body, .. }),
                    ..
                },
            ..
        } = event
        {
            msg_body
        } else {
            return;
        };
        if msg_body.contains(".rom") {
            let mut iter = msg_body.split(" ");
            //Skip the next iter
            iter.next();
            let phone = iter.collect::<Vec<&str>>().join(" ");
            let text = if let Some(device) = codename(phone.clone()) {
                format_device(device, vec![])
            } else if let Some((device, alternatives)) = search(phone) {
                format_device(device, alternatives)
            } else {
                "Phone not found".to_owned()
            };

            let content =
                AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(text));

            println!("sending");

            // send our message to the room we found the "!party" command in
            // the last parameter is an optional Uuid which we don't care about.
            room.send(content, None).await.unwrap();

            println!("message sent");
        }
    }
}

async fn on_stripped_state_member(
    room_member: StrippedStateEvent<MemberEventContent>,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().await.unwrap() {
        return;
    }

    if let Room::Invited(room) = room {
        println!("Autojoining room {}", room.room_id());
        let mut delay = 2;

        while let Err(err) = room.accept_invitation().await {
            // retry autojoin due to synapse sending invites, before the
            // invited user can join for more information see
            // https://github.com/matrix-org/synapse/issues/4345
            eprintln!(
                "Failed to join room {} ({:?}), retrying in {}s",
                room.room_id(),
                err,
                delay
            );

            sleep(Duration::from_secs(delay)).await;
            delay *= 2;

            if delay > 3600 {
                eprintln!("Can't join room {} ({:?})", room.room_id(), err);
                break;
            }
        }
        println!("Successfully joined room {}", room.room_id());
    }
}

pub async fn start_matrix() -> Result<()> {
    let client_config = ClientConfig::new().store_path("./store");

    let homeserver_url =
        Url::parse(&env::var("MATRIX_HOME").unwrap()).expect("Couldn't parse homeserver URL.");

    let client = Client::new_with_config(homeserver_url, client_config).unwrap();

    client
        .login(
            &env::var("MATRIX_USERNAME").unwrap(),
            &env::var("MATRIX_PASSWORD").unwrap(),
            None,
            Some("now rom"),
        )
        .await
        .unwrap();

    log::info!("Logged in as: {}", "now rom");
    client.register_event_handler(on_room_message).await;
    client
        .register_event_handler(on_stripped_state_member)
        .await;
    client.sync_once(SyncSettings::default()).await.unwrap();

    // client.register_event_handler(events::AnyRoomEvent).await;

    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    client.sync(settings).await;
    Ok(())
}
