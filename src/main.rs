use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
      
    let bot = Bot::from_env();

    println!("Bot is running!");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            bot.send_message(msg.chat.id, "Hi! 👋").await?;
        }

        respond(())
    })
    .await;
}
