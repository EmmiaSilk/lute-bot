
use super::CommandContext;
use super::CommandResult;
use super::common;

/// Stop music and remove Lute from the voice channel
#[poise::command(slash_command, rename = "leave")]
pub async fn command(
    ctx: CommandContext<'_>
) -> CommandResult<()> {
    let _guild = common::leave_channel(&ctx).await?;

    // TODO: Stop any currently-playing music
    ctx.reply("Goodbye!").await?;

    Ok(())
}
