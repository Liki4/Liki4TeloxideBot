use {
    crate::bot::{
        handler::command::{
            command_handler,
            Command,
        },
        util::MEDIA_GROUP_MAPPING,
    },
    std::{
        env,
        time::Duration,
    },
    teloxide::{
        prelude::{
            Message,
            ResponseResult,
        },
        respond,
        utils::command::BotCommands,
        Bot,
    },
    tokio::time::sleep,
};

pub async fn media_group_handler(msg: Message) -> ResponseResult<()> {
    MEDIA_GROUP_MAPPING.push_value(
        msg.media_group_id().unwrap(),
        msg.photo().unwrap().last().unwrap().file.id.clone(),
        Duration::from_secs(
            env::var("MEDIA_GROUP_MAPPING_TIMEOUT")
                .unwrap()
                .parse()
                .unwrap(),
        ),
    );
    respond(())
}

pub async fn media_group_with_command_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    media_group_handler(msg.clone()).await?;

    tokio::spawn(async move {
        sleep(Duration::from_secs(
            env::var("MEME_MEDIA_GROUP_HANDLE_TIMEOUT")
                .unwrap()
                .parse()
                .unwrap(),
        ))
        .await;

        if let Some(caption) = msg.caption() {
            if let Some(cmd) = Command::parse(caption, "").ok() {
                command_handler(bot, msg, cmd).await.ok();
            }
        }
    });

    respond(())
}
