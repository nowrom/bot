use twilight_http::Client;
use twilight_model::application::{
    command::{Command, CommandType},
    interaction::ApplicationCommand,
};
use twilight_util::builder::command::{CommandBuilder, StringBuilder};

use super::super::prelude::*;

pub fn command() -> Command {
    CommandBuilder::new(
        "embed".into(),
        "Set the embed to be used when uploading.".into(),
        CommandType::ChatInput,
    )
    .option(StringBuilder::new(
        "device".into(),
        "Device you want to search roms for".into(),
    ))
    .build()
}

pub async fn execute(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
    Ok(())
}
