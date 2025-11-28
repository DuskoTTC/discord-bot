use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;

#[poise::command(slash_command)]
pub async fn give_role(
    ctx: Context<'_>,
    member: serenity::Member,
    role: serenity::Role,
) -> Result<(), Error> {
    if member.roles.contains(&role.id) {
        ctx.send(
            poise::CreateReply::default()
                .content(format!(
                    "{} already has the role {}",
                    member.user.mention(),
                    role.mention()
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    match member.add_role(ctx, role.id).await {
        Ok(_) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(format!(
                        "Succesfully assigned role {} to {}!",
                        role.mention(),
                        member.user.mention()
                    ))
                    .ephemeral(true),
            )
            .await?;
        }
        Err(e) => {
            println!("Failed to assign role: {e:?}");
            ctx.send(
                poise::CreateReply::default()
                        .content("Error: Failed to assign role. Make sure I have the **Manage Roles** permission")
                        .ephemeral(true)
                ).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, subcommands("give_role"), subcommand_required)]
pub async fn moderation(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
