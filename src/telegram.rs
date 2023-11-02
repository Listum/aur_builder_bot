use teloxide::{prelude::*, utils::command::BotCommands};
use aur_rpc;
use git2::Repository;
use std::process::Command;
use std::env;


pub async fn main(bot: Bot){
    Commands::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands:")]
enum Commands{
    Upload(String),
    Clean,
    Search(String)
}
async fn answer(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
    match cmd {
        Commands::Upload(pkg) => {
            let repo = format!("https://aur.archlinux.org/{}.git", pkg);
            let dir = format!("pkgs/{}", pkg);
            match Repository::clone(repo.as_str(), dir.as_str()) {
                Ok(_) => bot.send_message(msg.chat.id, format!("Succsess {}", pkg)).await?,
                Err(e) => bot.send_message(msg.chat.id, format!("Error: {}", e)).await?,
            };
            env::set_current_dir(dir).expect("Failed to go to pkg dir");
            let output = Command::new("makepkg")
                .arg("-s")
                .arg("--noconfirm")
                .output()
                .expect("Failed to run makepkg");
            if output.status.success() {
                bot.send_message(msg.chat.id, format!("Build succseed")).await?;
            } else {
                bot.send_message(msg.chat.id, format!("Build failed")).await?;
            }
        }
        Commands::Clean => {
            bot.send_message(msg.chat.id, format!("Done")).await?;
        }
        Commands::Search(name) => {
                    let mut packages = aur_rpc::search(format!("{}", name)).await.unwrap();
                    packages.sort_by(|a, b| b.num_votes.cmp(&a.num_votes));
                    let mut result = Vec::new();
                    for (index, package) in packages.iter().enumerate().take(10) {
                        result.push(format!("{}. {}", index+1, package.name));
                    }
            bot.send_message(msg.chat.id, format!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"))).await?;
        }
    }
    Ok(())
}