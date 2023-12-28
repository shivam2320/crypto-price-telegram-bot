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
            let mut iter = token_name.split_whitespace();
            if let Some(crypto_symbol) = iter.next() {
                let target = to_uppercase(&format!("{}", &crypto_symbol));

                match market.get_price(target) {
                    Ok(symbol_price) => {
                        println!("{:#?}", &symbol_price);
                        bot.send_message(
                            msg.chat.id,
                            format!("The price you want is {:#?}. ", &symbol_price.price),
                        )
                        .await?
                    }
                    Err(e) => {
                        eprint!("{:#?}", e);

                        bot.send_message(
                            msg.chat.id,
                            format!(
                            "Something went wrong. Did you use the correct cryptocurrency pair?"
                        ),
                        )
                        .await?
                    }
                }
            } else {
                bot.send_message(msg.chat.id, format!("Cryptocurrency symbols were not specified. To start with, you can use /price ETH or /price ETH USDT."))
                .await?
            }
        }
    };

    Ok(())
}
