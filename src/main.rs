mod meme;

use meme_generator::meme::MemeInfo;
use teloxide::types::ParseMode;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::utils::markdown::escape;
use crate::meme::generator::{init_meme_mapping, init_resources};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    init_resources().await;
    init_meme_mapping().await;

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
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, escape(&Command::descriptions().to_string()))
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Dice => bot.send_dice(msg.chat.id).await?,
        Command::Meme { action, args } => meme::cmd::handler(&bot, &msg, action, args).await?,
    };
    Ok(())
}
