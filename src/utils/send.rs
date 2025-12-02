use crate::Error;
use poise::{CreateReply, ReplyHandle, async_trait};
use serenity::all::{Color, CreateEmbed, EditInteractionResponse};

pub fn make_info_embed(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
    CreateEmbed::new()
        .title(title)
        .description(description)
        .color(Color::BLUE)
}

pub fn make_warning_embed(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
    CreateEmbed::new()
        .title(title)
        .description(description)
        .color(Color::ORANGE)
}

pub fn make_error_embed(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
    CreateEmbed::new()
        .title(title)
        .description(description)
        .color(Color::RED)
}

#[async_trait]
pub trait ContextExt<'a> {
    async fn send_info(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<ReplyHandle<'a>, Error>;

    #[allow(dead_code)]
    async fn send_warning(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<ReplyHandle<'a>, Error>;

    async fn send_error(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<ReplyHandle<'a>, Error>;

    async fn update_info(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error>;

    async fn update_warning(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error>;

    async fn update_error(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error>;
}

#[async_trait]
impl<'a, U, E> ContextExt<'a> for poise::Context<'a, U, E>
where
    U: Sync + Send,
    E: Sync + Send,
{
    async fn send_info(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<ReplyHandle<'a>, Error> {
        self.send(CreateReply::default().embed(make_info_embed(title, description)))
            .await
            .map_err(Into::into)
    }

    async fn send_warning(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<ReplyHandle<'a>, Error> {
        self.send(CreateReply::default().embed(make_warning_embed(title, description)))
            .await
            .map_err(Into::into)
    }

    async fn send_error(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<ReplyHandle<'a>, Error> {
        self.send(CreateReply::default().embed(make_error_embed(title, description)))
            .await
            .map_err(Into::into)
    }

    async fn update_info(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error> {
        match self {
            poise::Context::Application(ctx) => {
                let embed = make_info_embed(&title.into(), &description.into());
                let builder = EditInteractionResponse::new().embed(embed);

                ctx.interaction.edit_response(ctx, builder).await?;
            }
            poise::Context::Prefix(_) => {
                self.send_info(title, description).await?;
            }
        }
        Ok(())
    }

    async fn update_warning(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error> {
        match self {
            poise::Context::Application(ctx) => {
                let embed = make_warning_embed(&title.into(), &description.into());
                let builder = EditInteractionResponse::new().embed(embed);

                ctx.interaction.edit_response(ctx, builder).await?;
            }
            poise::Context::Prefix(_) => {
                self.send_info(title, description).await?;
            }
        }
        Ok(())
    }

    async fn update_error(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error> {
        match self {
            poise::Context::Application(ctx) => {
                let embed = make_error_embed(&title.into(), &description.into());
                let builder = EditInteractionResponse::new().embed(embed);

                ctx.interaction.edit_response(ctx, builder).await?;
            }
            poise::Context::Prefix(_) => {
                self.send_error(title, description).await?;
            }
        }
        Ok(())
    }
}
