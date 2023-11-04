use thiserror::Error;

#[derive(Debug, Error)]
#[error("Not in a guild")]
pub struct NotInGuildError;

#[derive(Debug, Error)]
#[error("Did not receive songbird")]
pub struct NoSongbirdError;

#[derive(Debug, Error)]
#[error("Could not receive voice channel ID")]
pub struct NoVoiceChannelIdError;

#[derive(Debug, Error)]
#[error("You need to be in a voice channel")]
pub struct NotInVoiceChannelError;

#[derive(Debug, Error)]
#[error("No song was found")]
pub struct NoSongFoundError;

#[derive(Debug, Error)]
#[error("The given file path is illegal")]
pub struct IllegalFilePathError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Serenity(#[from] poise::serenity_prelude::Error),

    #[error(transparent)]
    NotInGuild(#[from] NotInGuildError),
    
    #[error(transparent)]
    NoSongbord(#[from] NoSongbirdError),

    #[error(transparent)]
    SongbirdJoin(#[from] songbird::error::JoinError),

    #[error(transparent)]
    NotInVoiceChannel(#[from] NotInVoiceChannelError),

    #[error(transparent)]
    NoVoiceChannelId(#[from] NoVoiceChannelIdError),

    #[error(transparent)]
    NoSongFound(#[from] NoSongFoundError),

    #[error(transparent)]
    SongbirdInput(#[from] songbird::input::error::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
    
    #[error(transparent)]
    IllegalFilePath(#[from] IllegalFilePathError),
}