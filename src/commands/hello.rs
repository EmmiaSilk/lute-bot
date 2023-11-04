use poise::serenity_prelude as serenity;

use super::CommandResult;
use super::CommandContext;

/// Recieve a greeting.
///
/// Provide the name of a user to greet them instead.
#[poise::command(slash_command, rename = "hello")]
pub async fn command(
    ctx: CommandContext<'_>,
    #[description = "User"] user: Option<serenity::User>,
) -> CommandResult<()> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("Hello {0}!", user);

    ctx.reply(response).await?;

    Ok(())
}
