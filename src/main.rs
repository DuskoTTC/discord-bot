use dashmap::DashMap;
use dotenv;
use poise::serenity_prelude as serenity;
use reqwest;
use songbird::serenity::SerenityInit;
use std::{env, sync::Arc};
use tokio::sync::Mutex;

use crate::structs::music_queue::MusicState;

mod commands;
mod structs;
mod utils;

pub struct Data {
    pub http_client: reqwest::Client,

    pub music_states: Arc<DashMap<serenity::all::GuildId, Arc<Mutex<MusicState>>>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let token =
        env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set in environment or .env file.");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_VOICE_STATES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            commands: commands::get_all_commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                println!("Bot is ready and commands are registered!");

                let http_client = reqwest::Client::new();

                let music_states = Arc::new(DashMap::new());

                Ok(Data {
                    http_client,
                    music_states,
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
