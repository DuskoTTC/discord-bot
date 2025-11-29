use crate::Error;
use poise::{CreateReply, async_trait};
use serenity::all::{Color, CreateEmbed};

#[async_trait]
pub trait ContextExt {
    async fn send_info(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error>;

    #[allow(dead_code)]
    async fn send_warning(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error>;

    async fn send_error(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error>;
}

#[async_trait]
impl<U, E> ContextExt for poise::Context<'_, U, E>
where
    U: Sync + Send,
    E: Sync + Send,
{
    async fn send_info(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error> {
        self.send(
            CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .title(title)
                        .description(description)
                        .color(Color::BLUE),
                )
                .reply(true),
        )
        .await?;
        Ok(())
    }

    async fn send_warning(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error> {
        self.send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title(format!("Warning: {}", title.into()))
                    .description(description)
                    .color(Color::ORANGE),
            ),
        )
        .await?;

        Ok(())
    }

    async fn send_error(
        &self,
        title: impl Into<String> + Send,
        description: impl Into<String> + Send,
    ) -> Result<(), Error> {
        self.send(
            poise::CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .title(format!("Error: {}", title.into()))
                        .description(description)
                        .color(Color::RED),
                )
                .ephemeral(true)
                .reply(true),
        )
        .await?;
        Ok(())
    }
}
