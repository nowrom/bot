use std::env;
use std::sync::Arc;

use futures::StreamExt;
use twilight_gateway::cluster::ShardScheme;
use twilight_gateway::Cluster;
use twilight_gateway::Event;
use twilight_gateway::Intents;
use twilight_http::Client;
use twilight_model::application::callback::Autocomplete;
use twilight_model::application::callback::InteractionResponse;
use twilight_model::application::command::CommandOptionChoice;
use twilight_model::application::interaction::Interaction;
use twilight_model::id::ApplicationId;
use twilight_model::id::GuildId;

use crate::discord::commands;
use crate::discord::prelude::*;
use crate::search;

use super::commands::rom;
use super::commands::rom::get_arg;

#[allow(clippy::single_match)]
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
                    if let Err(_r) = r {}
                }
                Interaction::ApplicationCommandAutocomplete(cmd) => {
                    let iter = cmd.data.options.iter();
                    let device = get_arg(iter.clone(), "device");
                    let r = search(device.unwrap()).await;
                    http.interaction_callback(
                        cmd.id,
                        &cmd.token,
                        &InteractionResponse::Autocomplete(Autocomplete {
                            choices: r
                                .map(|x| {
                                    let mut devices = x.1;
                                    devices
                                        .into_iter()
                                        .map(|x| CommandOptionChoice::String {
                                            value: x.codename,
                                            name: x.name,
                                        })
                                        .collect::<Vec<CommandOptionChoice>>()
                                })
                                .unwrap_or(Vec::new()),
                        }),
                    )
                    .exec()
                    .await
                    .unwrap();
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
