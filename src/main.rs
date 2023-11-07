mod telegram;
mod build;
mod search;
mod authorization;

#[tokio::main]
async fn main() {

    telegram::main(teloxide::Bot::from_env()).await;
}
