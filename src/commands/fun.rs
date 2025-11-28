use crate::{Context, Error};
use poise::command;

#[command(prefix_command, slash_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;
    let response = format!("Pong! Latency: `{}ms`", latency.as_millis());

    ctx.reply(response).await?;
    Ok(())
}
