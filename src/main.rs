mod telegram;
mod build;
mod search;

#[tokio::main]
async fn main() {

    telegram::main(teloxide::Bot::from_env()).await;
}
