use {
    crate::meme,
    teloxide::{
        payloads::{
            SendDiceSetters,
            SendMessageSetters,
        },
        prelude::{
            Message,
            Requester,
            ResponseResult,
        },
        types::ReplyParameters,
        utils::command::BotCommands,
        Bot,
    },
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display help.")]
    Help,
    #[command(description = "roll a dice.")]
    Dice,
    #[command(description = "generate memes.", parse_with = meme::parser)]
    Meme {
        action: meme::MemeAction,
        args: Vec<String>,
    },
}

pub async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    tokio::spawn(async move {
        let _ = match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, &Command::descriptions().to_string())
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await
            }
            Command::Dice => {
                bot.send_dice(msg.chat.id)
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await
            }
            Command::Meme { action, args } => {
                meme::cmd::meme_command_handler(&bot, &msg, action, args).await
            }
        };
    });
    Ok(())
}
