use binance::api::*;
use binance::market::*;
use teloxide::{prelude::*, utils::command::BotCommands};

fn to_uppercase(string: &str) -> String {
    string.chars().map(|c| c.to_ascii_uppercase()).collect()
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Welcome to Crypto Price Bot")]
    Start,
    #[command(description = "Get price of token in USDT")]
    Token(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Token(token_name) => {
            let market: Market = Binance::new(None, None);

            let default = "USDT";

            let target = to_uppercase(&format!("{}{}", &token_name, &default));

            match market.get_price(target) {
                Ok(symbol_price) => {
                    println!("{:#?}", &symbol_price);
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "Price of {} is {:#?} USDT ",
                            &token_name, &symbol_price.price
                        ),
                    )
                    .await?
                }
                Err(e) => {
                    eprint!("{:#?}", e);

                    bot.send_message(msg.chat.id, format!("Incorrect symbol"))
                        .await?
                }
            }
        }
    };

    Ok(())
}
