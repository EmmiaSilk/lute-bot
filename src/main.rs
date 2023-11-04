use std::path::Path;

mod commands;
mod framework;
mod error;

#[tokio::main]
async fn main() {
    // Login to discord
    let token = std::env::var("DISCORD_TOKEN").expect("Token");
    let music_dir = std::env::var("MUSIC_DIR").expect("Music Directory");
    let music_dir = Path::new(&music_dir).canonicalize().expect("Valid canon music dir");

    let framework = framework::build(token, music_dir);

    framework.run().await.unwrap();
}
