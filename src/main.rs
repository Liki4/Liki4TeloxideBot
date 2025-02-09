mod meme;

use std::env;
use {
    crate::meme::{
        generator::{init_meme_mapping, init_resources},
        MEDIA_GROUP_MAPPING,
    },
    std::time::Duration,
    teloxide::{prelude::*, types::ReplyParameters, utils::command::BotCommands},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    pretty_env_logger::init();

    init_resources().await;
    init_meme_mapping().await;

    log::info!("Starting command bot...");
    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(answer))
        .branch(
            dptree::filter(|msg: Message| {
                // log::debug!("{:?}", msg);
                msg.media_group_id().is_some() && msg.photo().is_some()
            })
            .endpoint(|msg: Message| async move {
                MEDIA_GROUP_MAPPING.push_value(
                    msg.media_group_id().unwrap(),
                    msg.photo().unwrap().last().unwrap().file.id.clone(),
                    Duration::from_secs(env::var("MEME_MEDIA_GROUP_MAPPING_TIMEOUT").unwrap().parse().unwrap()),
                );
                respond(())
            }),
        );

    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            log::trace!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
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
    #[command(description = "generate memes.", parse_with = meme::parser)]
    Meme {
        action: meme::MemeAction,
        args: Vec<String>,
    },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
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
            Command::Meme { action, args } => meme::cmd::handler(&bot, &msg, action, args).await,
        };
    });
    Ok(())
}
