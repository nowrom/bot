use twilight_http::Client;
use twilight_model::application::interaction::ApplicationCommand;

use super::prelude::*;

pub async fn builtin_exec(client: &Client, cmd: &ApplicationCommand) -> Result<()> {
    let value = match cmd.data.name.as_str() {
        "rom" => rom::execute(client, cmd).await,
        _ => return Ok(()),
    };

    match value {
        Ok(_) => {}
        Err(_err) => {}
    };
    Ok(())
}
pub mod rom;
