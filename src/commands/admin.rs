use crate::{Context, Error};
use poise::macros::command;

#[command(
    prefix_command,
    guild_only,
    subcommands("register"),
    subcommand_required
)]
pub async fn admin(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
