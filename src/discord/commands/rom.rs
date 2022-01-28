use std::slice::Iter;
use twilight_http::Client;
use twilight_model::application::callback::InteractionResponse;
use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::interaction::{
    application_command::{CommandDataOption, CommandOptionValue},
    ApplicationCommand,
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    CallbackDataBuilder,
};

use crate::{codename, format_device, search};

use super::super::prelude::*;

pub fn command() -> Command {
    CommandBuilder::new(
        "rom".into(),
        "Find the fucking rom".into(),
        CommandType::ChatInput,
    )
    .option(
        StringBuilder::new("device".into(), "Device you want to search roms for".into())
            .autocomplete(true),
    )
    .option(StringBuilder::new(
        "codename".into(),
        "Find a device by codename".into(),
    ))
    .build()
}

pub fn get_arg(mut args: Iter<CommandDataOption>, key: &str) -> Option<String> {
    let domain = args.find(|e| e.name == key);
    if let Some(domain) = domain {
        match &domain.value {
            CommandOptionValue::String(val) => Some(val.clone()),
            _ => None,
        }
    } else {
        None
    }
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
    let iter = cmd.data.options.iter();
    let device = get_arg(iter.clone(), "device");
    let code = get_arg(iter.clone(), "codename");
    let cdn = device.map(|x| Some(x)).unwrap_or(code);
    let m = if let Some(device) = cdn {
        let device = codename(device).await;
        if let Some(device) = device {
            format_device(device, Vec::new())
        } else {
            "Phone not found".to_owned()
        }
    } else {
        "Please provide either a device or a codename".to_owned()
    };

    client
        .interaction_callback(
            cmd.id,
            &cmd.token,
            &InteractionResponse::ChannelMessageWithSource(
                CallbackDataBuilder::new().content(m).build(),
            ),
        )
        .exec()
        .await?;

    Ok(())
}
