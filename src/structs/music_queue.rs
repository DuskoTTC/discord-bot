use serenity::all::UserId;
use songbird::tracks::TrackHandle;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub title: String,
    pub channel: String,
    pub url: String,
    pub thumbnail: String,
    pub duration: std::time::Duration,
    pub requester: UserId,
}

#[derive(Debug)]
pub struct QueueItem {
    pub handle: TrackHandle,
    pub info: TrackMetadata,
}

#[derive(Debug, Default)]
pub struct MusicState {
    pub current_track: Option<QueueItem>,
    pub queue: VecDeque<QueueItem>,
    pub loop_mode: QueueMode,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum QueueMode {
    #[default]
    Normal, // Play -> Remove -> Next
    Loop, // Play -> Move to Back -> Next
}
