use crate::structs::music_queue::{MusicState, QueueItem, TrackMetadata};
use crate::{Context, Error};
use poise::{CreateReply, async_trait, macros::command};
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use songbird::input::{Compose, YoutubeDl};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::utils::send::ContextExt;

#[command(
    slash_command,
    guild_only,
    subcommands("play", "join", "leave", "deafen"),
    subcommand_required
)]
pub async fn music(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[command(slash_command, ephemeral)]
async fn play(
    ctx: Context<'_>,
    #[description = "A search query or YouTube URL"] query_or_url: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let (guild_id, channel_id) = {
        let guild = ctx.guild().expect("This command is guild only.");

        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vs| vs.channel_id);

        (guild.id, channel_id)
    };

    let channel_id = match channel_id {
        Some(id) => id,
        None => {
            ctx.send_error("You must be in a voice channel to use this command.")
                .await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in setup.")
        .clone();

    let handler_lock = match manager.join(guild_id, channel_id).await {
        Ok(handler) => handler,
        Err(e) => {
            ctx.send_error(format!("Error getting track: {e:?}"))
                .await?;
            return Ok(());
        }
    };

    let mut handler = handler_lock.lock().await;
    handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);

    let is_url = query_or_url.starts_with("http://") || query_or_url.starts_with("https://");

    let http_client = ctx.data().http_client.clone();

    let mut source = if is_url {
        YoutubeDl::new(http_client, query_or_url.clone())
    } else {
        YoutubeDl::new_search(http_client, query_or_url.clone())
    };

    let state_map = &ctx.data().music_states;
    let music_state = state_map
        .entry(guild_id)
        .or_insert_with(|| Arc::new(Mutex::new(MusicState::default())))
        .clone();

    let metadata = match source.aux_metadata().await {
        Ok(metadata) => metadata,
        Err(e) => {
            ctx.send_error(format!("Error fetching video metadata: {e}"))
                .await?;
            return Ok(());
        }
    };

    let track_handle = handler.play_input(source.into());

    track_handle.add_event(
        songbird::Event::Track(songbird::TrackEvent::End),
        TrackEndHandler {
            state: music_state.clone(),
        },
    )?;

    let title = metadata
        .title
        .unwrap_or_else(|| "Unknown title".to_string());
    let channel = metadata
        .channel
        .unwrap_or_else(|| "Unknown artist".to_string());
    let url = metadata.source_url.unwrap_or_default();
    let thumbnail = metadata
        .thumbnail
        .unwrap_or_else(|| "https://media.istockphoto.com/id/1147544809/vector/no-thumbnail-image-vector-graphic.jpg?s=170667a&w=0&k=20&c=v9QBkaN6fXxy1b-wsTQ6QhHUVGLo8JMMxhUBcWzOH0A=".to_string());
    let duration = metadata.duration.unwrap_or_default();

    let track_metadata = TrackMetadata {
        title: title.clone(),
        channel: channel.clone(),
        url,
        thumbnail,
        duration,
        requester: ctx.author().id,
    };

    let queue_item = QueueItem {
        handle: track_handle,
        info: track_metadata,
    };

    let mut state = music_state.lock().await;

    if state.current_track.is_none() {
        state.current_track = Some(queue_item);
    } else {
        state.queue.push_back(queue_item);
    }

    ctx.reply(format!("Playing: **{}** by **{}**", title, channel))
        .await?;

    Ok(())
}

#[command(slash_command, ephemeral)]
async fn deafen(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.send(
                CreateReply::default()
                    .content("Not in a voice channel")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        ctx.send(
            CreateReply::default()
                .content("Already deafened")
                .ephemeral(true),
        )
        .await?;
    } else {
        if let Err(e) = handler.deafen(true).await {
            ctx.send(
                CreateReply::default()
                    .content(format!("Failed: {:?}", e))
                    .ephemeral(true),
            )
            .await?;
        } else {
            ctx.send(CreateReply::default().content("Deafened").ephemeral(true))
                .await?;
        }
    }

    Ok(())
}

#[command(slash_command, ephemeral)]
async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vs| vs.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.send_error("You must be in a voice channel to use this command")
                .await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client. Placed in at initialisation.")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    }

    ctx.send_embed("", "Joined voice channel").await?;

    Ok(())
}

#[command(slash_command, ephemeral)]
async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            ctx.send_error(format!("Failed: {:?}", e)).await?;
        }

        ctx.send_embed("", "Left voice channel").await?;
    }

    Ok(())
}

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track{:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}

pub struct TrackEndHandler {
    pub state: Arc<Mutex<MusicState>>,
}

#[async_trait]
impl VoiceEventHandler for TrackEndHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_track_event) = ctx {
            let mut state = self.state.lock().await;

            if let Some(next_item) = state.queue.pop_front() {
                let _ = next_item.handle.play();
                state.current_track = Some(next_item);
            } else {
                state.current_track = None;
            }
        }
        None
    }
}
