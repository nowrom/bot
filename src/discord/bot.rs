use std::env;
use std::sync::Arc;

use futures::StreamExt;
use twilight_gateway::cluster::ShardScheme;
use twilight_gateway::Cluster;
use twilight_gateway::Event;
use twilight_gateway::Intents;
use twilight_http::Client;
use twilight_model::application::callback::CallbackData;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::AllowedMentions;
use twilight_model::id::ApplicationId;
use twilight_model::id::GuildId;

use crate::discord::commands;
use crate::discord::prelude::*;

use super::commands::rom;

pub async fn start_discord() -> Result<()> {
    dotenv::dotenv().ok();

    let token = dotenv::var("DISCORD_TOKEN")?;

    let (cluster, mut events) = Cluster::builder(&*token, Intents::GUILD_MESSAGES)
        .shard_scheme(ShardScheme::Auto)
        .build()
        .await?;

    cluster.up().await;

    let http = Arc::new(Client::new((&*token).to_string()));
    http.set_application_id(ApplicationId(
        env::var("DISCORD_APPLICATION_ID")
            .unwrap()
            .parse::<core::num::NonZeroU64>()
            .unwrap(),
    ));

    http.set_guild_commands(
        GuildId(core::num::NonZeroU64::new(748956745409232945).unwrap()),
        &[rom::command()],
    )
    .unwrap()
    .exec()
    .await?;

    while let Some((_shard_id, event)) = events.next().await {
        match event {
            Event::InteractionCreate(inter) => match inter.0 {
                Interaction::ApplicationCommand(cmd) => {
                    log::info!(
                        "slash command called {} {}",
                        cmd.data.name,
                        cmd.member.as_ref().unwrap().user.as_ref().unwrap().name
                    );
                    let r = commands::builtin_exec(&http, &cmd).await;
                    if let Err(r) = r {}
                }
                _ => {}
            },
            Event::Ready(_) => {
                log::info!("Bot got on")
            }
            _ => {}
        }
    }

    Ok(())
}
