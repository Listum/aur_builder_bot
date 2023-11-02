use teloxide::{prelude::*, utils::command::BotCommands};
use aur_rpc;
use git2::Repository;
use std::process::Command;
use std::env;
use glob::glob;
use std::fs;
use std::path::Path;



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

            let default_dir = env::current_dir().expect("Failed to get default directory");
            env::set_current_dir(dir.clone())?;
            let compile = Command::new("makepkg")
                .arg("-s")
                .arg("--noconfirm")
                .output()?;

            if compile.status.success() {
                bot.send_message(msg.chat.id, format!("Build succseed")).await?;
            } else {
                bot.send_message(msg.chat.id, format!("Build failed: {:?}", compile.status.to_string())).await?;
            }

            env::set_current_dir(default_dir.clone())?;
            let local_repo = "repo/";
            let entries = fs::read_dir(dir.clone())?;

            for entry in entries {
                let entry = entry?;
                let file_path = entry.path();

                if file_path.is_file() && file_path.extension().unwrap_or_default() == "zst" {
                    let file_name = file_path.file_name().unwrap();
                    let local_repo = Path::new(local_repo).join(file_name);
                    fs::copy(&file_path, &local_repo).expect("Failed to copy");
                    bot.send_message(msg.chat.id, format!("Package copied to local repo")).await?;
                }
            }

            match fs::remove_dir_all(dir) {
                Ok(_) => println!("Sources successfully deleted."),
                Err(err) => eprintln!("Error deleting sources: {}", err),
            }

            env::set_current_dir("repo/")?;
            let files: Vec<String> = glob("*.pkg.tar.zst")
                .expect("Failed to read glob pattern")
                .filter_map(|entry| entry.ok())
                .filter_map(|path| path.to_str().map(String::from))
                .collect();

            if files.is_empty() {
                println!("No matching files found.");
            }

            let vec_of_str_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
            let add = Command::new("repo-add")
                .arg("repo.db.tar.gz")
                .arg(format!("{}", vec_of_str_refs.join(", ")))
                .output()?;
            eprintln!("repo-add output: {:?}", String::from_utf8_lossy(&add.stderr));

            if add.status.success() {
                bot.send_message(msg.chat.id, format!("Added to repo successfuly")).await?;
            } else {
                bot.send_message(msg.chat.id, format!("Failed to add to repo: {:?}", compile.status.code())).await?;
            }
            env::set_current_dir(default_dir)?;
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