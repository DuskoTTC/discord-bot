pub mod admin;
pub mod fun;
pub mod moderation;
pub mod music;

use crate::{Data, Error};

pub fn get_all_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        fun::ping(),
        admin::admin(),
        moderation::moderation(),
        music::music(),
    ]
}
