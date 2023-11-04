use crate::{error::Error, framework::Config};

mod common;
mod hello;
mod play;
mod leave;

pub type CommandContext<'a> = poise::Context<'a, Config, Error>;
pub type CommandResult<R> = core::result::Result<R, Error>;

pub fn get() -> Vec<poise::Command<Config, Error>> {
    vec![
        hello::command(),
        play::command(),
        leave::command(),
    ]
}
