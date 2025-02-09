use {
    log::info,
    md5::{Digest, Md5},
    std::collections::HashMap,
    teloxide::{
        net::Download,
        prelude::*,
        types::{
            FileMeta, InputFile,
            MessageEntityKind::{Mention, TextMention},
            ParseMode, ReplyParameters, User,
        },
    },
};

pub fn hash_short(filename: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(filename);
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);
    let final_hash = &hex_hash[0..8];
    final_hash.to_string()
}

async fn file_downloader(bot: &Bot, file_meta: &FileMeta) -> ResponseResult<(String, Vec<u8>)> {
    let file = bot.get_file(&file_meta.id).await?;
    let file_id = file_meta.id.clone();
    let mut file_vec: Vec<u8> = Vec::new();
    bot.download_file(file.path.as_str(), &mut file_vec).await?;
    Ok((file_id, file_vec))
}

pub async fn get_sender_profile_photo(
    bot: &Bot,
    msg: &Message,
) -> ResponseResult<Option<(String, Vec<u8>)>> {
    let user = match &msg.from {
        Some(sender) => sender,
        None => {
            bot.send_message(msg.chat.id, "error: msg sender not found")
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
            return Ok(None);
        }
    };
    let profile_photos = bot.get_user_profile_photos(user.id).await?;
    if let Some(profile_photo_list) = profile_photos.photos.first() {
        let profile_photo = profile_photo_list.last().unwrap();
        let (file_id, file_data) = file_downloader(bot, &profile_photo.file).await?;
        Ok(Some((file_id, file_data)))
    } else {
        bot.send_message(
            msg.chat.id,
            &format!("error: user `{}` has no profile photo", user.first_name),
        )
        .parse_mode(ParseMode::MarkdownV2)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
        Ok(None)
    }
}

pub async fn get_final_photo_list(
    bot: &Bot,
    msg: &Message,
) -> ResponseResult<Option<Vec<(String, Vec<u8>)>>> {
    let mut final_photo_list = Vec::<(String, Vec<u8>)>::new();

    let entities = msg.entities().unwrap_or_default();
    let msg_mentioned_users: HashMap<String, User> = msg
        .mentioned_users()
        .filter_map(|u| {
            u.username
                .as_ref()
                .map(|username| (username.clone(), u.clone()))
        })
        .collect();

    for e in entities {
        if let Some(text) = msg.text() {
            let user_text = text
                .chars()
                .skip(e.offset)
                .take(e.length)
                .collect::<String>();
            let user: User = match e.to_owned().kind {
                Mention => match msg_mentioned_users.get(&user_text[1..]) {
                    Some(user) => user.to_owned(),
                    None => {
                        bot.send_message(
                            msg.chat.id,
                            &format!("error: user `{user_text}` not found"),
                        )
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_parameters(ReplyParameters::new(msg.id))
                        .await?;
                        return Ok(None);
                    }
                },
                TextMention { user } => user.to_owned(),
                _ => continue,
            };
            let profile_photos = bot.get_user_profile_photos(user.id).await?;
            if let Some(profile_photo_list) = profile_photos.photos.first() {
                let profile_photo = profile_photo_list.last().unwrap();
                let (id, data) = file_downloader(bot, &profile_photo.file).await?;
                final_photo_list.push((id, data));
            } else {
                bot.send_message(
                    msg.chat.id,
                    &format!("error: user `{}` has no profile photo", user_text),
                )
                .parse_mode(ParseMode::MarkdownV2)
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
                return Ok(None);
            }
        };
    }

    if let Some(replied_msg) = msg.reply_to_message() {
        if let Some(photos) = replied_msg.photo() {
            let replied_photo = photos.last().unwrap();
            let (id, data) = file_downloader(bot, &replied_photo.file).await?;
            final_photo_list.push((id, data));
        }
        if let Some(animation) = replied_msg.animation() {
            let (id, data) = file_downloader(bot, &animation.file).await?;
            final_photo_list.push((id, data));
        }
    }

    if let Ok(sender_profile_photo) = get_sender_profile_photo(bot, msg).await {
        if let Some((id, data)) = sender_profile_photo {
            final_photo_list.push((id, data));
        }
    }

    Ok(Some(final_photo_list))
}

pub async fn send_media(
    bot: &Bot,
    msg: &Message,
    content: Vec<u8>,
    filename: String,
    caption: Option<String>,
) -> ResponseResult<Message> {
    info!("sending media file: {}B", content.len());
    match infer::get(&content) {
        Some(mime_type) => {
            let extension = mime_type.extension();
            let input_file =
                InputFile::memory(content.clone()).file_name(format!("{filename}.{extension}"));

            match mime_type.matcher_type() {
                infer::MatcherType::Image => match mime_type.mime_type() {
                    "image/gif" | "image/webp" => {
                        let mut action = bot
                            .send_animation(msg.chat.id, input_file)
                            .parse_mode(ParseMode::MarkdownV2)
                            .reply_parameters(ReplyParameters::new(msg.id));
                        action.caption = caption;
                        action.await
                    }
                    _ => {
                        let mut action = bot
                            .send_photo(msg.chat.id, input_file)
                            .parse_mode(ParseMode::MarkdownV2)
                            .reply_parameters(ReplyParameters::new(msg.id));
                        action.caption = caption;
                        action.await
                    }
                },
                infer::MatcherType::Video => {
                    let mut action = bot
                        .send_video(msg.chat.id, input_file)
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_parameters(ReplyParameters::new(msg.id));
                    action.caption = caption;
                    action.await
                }
                _ => {
                    bot.send_message(msg.chat.id, "error: File type is not media")
                        .reply_parameters(ReplyParameters::new(msg.id))
                        .await
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "error: File type unknown")
                .reply_parameters(ReplyParameters::new(msg.id))
                .await
        }
    }
}
