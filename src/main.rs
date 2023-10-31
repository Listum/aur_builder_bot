mod telegram;

use teloxide;


#[tokio::main]
async fn main() {
    telegram::main(teloxide::Bot::from_env()).await;
}
