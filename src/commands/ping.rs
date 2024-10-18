use poise::serenity_prelude as serenity;
use crate::{Context, Error}; // Assuming you define `Context` and `Error` in the root module

/// Ping command implementation
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong! 🏓").await?;
    Ok(())
}
