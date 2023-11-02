use teloxide::{prelude::*, utils::command::BotCommands};
use aur_rpc;
use git2::Repository;


pub async fn main(bot: Bot){
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands:")]
enum Command{
    Upload(String),
    Clean,
    Search(String)
}
async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Upload(pkg) => {
            let repo = format!("https://aur.archlinux.org/{}.git", pkg);
            let dir = format!("pkgs/{}", pkg);
            match Repository::clone(repo.as_str(), dir.as_str()) {
                Ok(_) => bot.send_message(msg.chat.id, format!("Succsess {}", pkg)).await?,
                Err(e) => bot.send_message(msg.chat.id, format!("Error: {}", e)).await?,
            };

        }
        Command::Clean => {
            bot.send_message(msg.chat.id, format!("Done")).await?;
        }
        Command::Search(name) => {
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