use std::path::PathBuf;

use crate::commands;
use crate::error::Error;

use poise::serenity_prelude as serenity;
use songbird::SerenityInit;

/// User data, which is stored and accessible in all command invocations
pub struct Config {
    music_dir: PathBuf,
}
impl Config {
    pub fn music_dir(&self) -> &PathBuf {
        &self.music_dir
    }
}

pub fn build(token: String, music_dir: PathBuf) -> poise::FrameworkBuilder<Config, Error> {
    let intents = serenity::GatewayIntents::non_privileged();
    let options = poise::FrameworkOptions {
        commands: commands::get(),
        ..Default::default()
    };

    let config = Config {
        music_dir,
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(token)
        .intents(intents)
        .client_settings(|c| c.register_songbird())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(config)
            })
        });

    framework
}