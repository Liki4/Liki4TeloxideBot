mod meme;

use crate::meme::generator::{init_meme_mapping, init_resources};
use teloxide::types::{ParseMode, ReplyParameters};
use teloxide::utils::markdown::escape;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    init_resources().await;
    init_meme_mapping().await;

    log::info!("Starting command bot...");
    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(answer));

    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd.id);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    // Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display help.")]
    Help,
    #[command(description = "roll a dice.")]
    Dice,
    #[command(description = "generate memes. photo>mention>reply", parse_with = meme::parser)]
    Meme {
        action: meme::MemeAction,
        args: Vec<String>,
    },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    tokio::spawn(async move {
        let _ = match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, escape(&Command::descriptions().to_string()))
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await
            }
            Command::Dice => {
                bot.send_dice(msg.chat.id)
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await
            }
            Command::Meme { action, args } => meme::cmd::handler(&bot, &msg, action, args).await,
        };
    });
    Ok(())
}
