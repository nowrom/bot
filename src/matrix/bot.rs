use std::env;

use anyhow::Result;
use matrix_sdk::{ruma::events, Client, ClientConfig, SyncSettings};
use reqwest::Url;

pub async fn start_matrix() {
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

    client.sync_once(SyncSettings::default()).await.unwrap();

    // client.register_event_handler(events::AnyRoomEvent).await;

    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    client.sync(settings).await;
}
