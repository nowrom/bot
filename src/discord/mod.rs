pub mod bot;
pub mod commands;
pub mod prelude {
    pub use anyhow::Result;
    pub use twilight_model::application::callback::ResponseType::ChannelMessageWithSource;
}
