use super::common;
use super::CommandContext;
use super::CommandResult;

/// Invite Lute to the voice channel
#[poise::command(slash_command, rename = "play")]
pub async fn command(ctx: CommandContext<'_>, uri: String) -> CommandResult<()> {
    ctx.defer().await?;

    let (_, _, _, call) = common::join_channel(&ctx).await?;

    let source = common::retrieve_source(&ctx, uri.clone()).await?;

    let handle = call.lock().await.play_only_source(source);

    ctx.reply(format!("Playing {0}", uri)).await?;

    Ok(())
}
