use std::sync::Arc;

use poise::serenity_prelude as serenity;
use serenity::ChannelId;
use serenity::GuildId;
use songbird::Call;
use songbird::Songbird;
use songbird::input::Input;
use songbird::input::Restartable;
use tokio::sync::Mutex;

use super::CommandContext;
use super::CommandResult;

use crate::error;
use crate::error::IllegalFilePathError;
use crate::error::NoSongbirdError;
use crate::error::NoVoiceChannelIdError;
use crate::error::NotInGuildError;
use crate::error::NotInVoiceChannelError;

/// Tell Lute to join the channel that the sender is currently in.
pub async fn join_channel(
    ctx: &CommandContext<'_>,
) -> CommandResult<(GuildId, ChannelId, Arc<Songbird>, Arc<Mutex<Call>>)> {
    let guild = ctx.guild().ok_or(NotInGuildError)?;

    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .ok_or(NotInVoiceChannelError)?
        .channel_id
        .ok_or(NoVoiceChannelIdError)?;

    let songbird_manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;

    
    let (call, result) = songbird_manager.join(guild.id, channel_id).await;
    if let Err(err) = result {
        ctx.reply("I'm having some kind of unexplained issues.").await?;
        if err.should_leave_server() {
            call.lock().await.leave().await?;
        }
        return Err(err.into());
    }

    Ok((guild.id, channel_id, songbird_manager, call))
}

/// Tell Lute to leave the voice channel she is currently in.
pub async fn leave_channel(
    ctx: &CommandContext<'_>,
) -> CommandResult<GuildId> {
    let guild_id = ctx.guild_id().ok_or(NotInGuildError)?;

    let songbird_manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;

    songbird_manager.leave(guild_id).await?;

    Ok(guild_id)
}

/// Retrieve a sound source, either from youtube or from a local file.
pub async fn retrieve_source(ctx: &CommandContext<'_>, uri: String) -> CommandResult<Input> {
    let music_dir = ctx.data().music_dir();

    // Disallow absolute paths
    if uri.starts_with("/") || uri.starts_with("file://") {
        return Err(IllegalFilePathError.into());
    }

    // Online paths
    let source: Input;
    if uri.starts_with("http://") || uri.starts_with("https://") {
        source = songbird::input::ytdl(uri).await?;
    }
    else {
        // Relative Paths
        let relative_path = clean_path::clean(uri); // Must clean for safety
        let absolute_path = music_dir.join(relative_path);

        // Allow only music in the music directory
        if !absolute_path.starts_with(music_dir) {
            return Err(IllegalFilePathError.into());
        }
        source = Restartable::ffmpeg(absolute_path, true)
            .await?
            .into();
    }

    Ok(source)
}