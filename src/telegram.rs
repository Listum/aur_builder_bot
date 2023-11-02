use teloxide::{prelude::*, utils::command::BotCommands};
use crate::build::{clone, copy, build, delete, repo_add};
use crate::search;
use std::env;
pub async fn main(bot: Bot){
    Commands::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands:")]
pub(crate) enum Commands{
    Upload(String),
    Search(String)
}
async fn answer(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
    match cmd {
        Commands::Upload(pkg) => {
            let default_dir = env::current_dir()?;
            let pkg_dir = format!("pkgs/{}", pkg);
            let repo_dir = format!("repo/");

            let clone = match clone(pkg, pkg_dir.clone()) {
                Ok(..) => { format!("Cloned") },
                Err(e) => { format!("Clone error: {}", e) }
            };
            bot.send_message(msg.chat.id, clone).await?;

            env::set_current_dir(pkg_dir.clone())?;
            let build = match build().await {
                Ok(..) => { format!("Builded") },
                Err(e) => { format!("Build error: {}", e) }
            };
            bot.send_message(msg.chat.id, build).await?;

            env::set_current_dir(default_dir.clone())?;
            let copy = match copy(pkg_dir.clone(), repo_dir.clone()).await {
                Ok(..) => { format!("Copied") },
                Err(e) => { format!("Copy error: {}", e) }
            };
            bot.send_message(msg.chat.id, copy).await?;

            let delete = match delete(pkg_dir) {
                Ok(..) => { format!("Sources deleted")},
                Err(e) => { format!("Sources deletion error: {}", e) }
            };
            bot.send_message(msg.chat.id, delete).await?;

            env::set_current_dir(repo_dir)?;
            let repo_add = match repo_add().await {
                Ok(..) => { format!("Added to repo") },
                Err(e) => { format!("Error adding package to repository: {}", e) }
            };
            bot.send_message(msg.chat.id, repo_add).await?;
            env::set_current_dir(default_dir)?;

        }
        Commands::Search(pkg) => {
            bot.send_message(msg.chat.id, search::search(pkg).await).await?;
        }
    }
    Ok(())
}