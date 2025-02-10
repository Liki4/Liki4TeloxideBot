mod bot;
mod meme;

use {
    crate::bot::{
        bot_init,
        handler::endpoint::{
            media_group_handler,
            media_group_with_command_handler,
        },
    },
    bot::handler::{
        command,
        command::Command,
    },
    teloxide::{
        prelude::*,
        utils::command::BotCommands,
    },
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    pretty_env_logger::init();

    bot_init().await;

    log::info!("Starting command bot...");
    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(command::command_handler),
        )
        .branch(
            dptree::filter(|msg: Message| msg.media_group_id().is_some() && msg.photo().is_some())
                .branch(
                    dptree::filter(|msg: Message| {
                        msg.caption()
                            .map_or(false, |caption| Command::parse(caption, "").is_ok())
                    })
                    .endpoint(media_group_with_command_handler),
                )
                .branch(dptree::endpoint(media_group_handler)),
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
