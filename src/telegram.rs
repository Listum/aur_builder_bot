use teloxide::{prelude::*, utils::command::BotCommands};
use aur_rpc;

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
        Command::Upload(url) => {
            bot.send_message(msg.chat.id, format!("Test {}", url)).await?;
        }
        Command::Clean => {
            bot.send_message(msg.chat.id, format!("Done")).await?;
        }
        Command::Search(name) => {
                    let packages = aur_rpc::search(format!("{}", name)).await.unwrap();
                    let mut sorted_packages = packages;
                    sorted_packages.sort_by(|a, b| b.num_votes.cmp(&a.num_votes));
                    let mut result:Vec<String> = Vec::new();
                    for (index, package) in sorted_packages.iter().enumerate().take(10) {
                        result.push(format!("{}. {}\n", index+1, package.name));
                    }
            bot.send_message(msg.chat.id, format!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""))).await?;
        }
    }
    Ok(())
}