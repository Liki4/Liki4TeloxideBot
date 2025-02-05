use std::collections::HashMap;
use teloxide::net::Download;
use teloxide::prelude::*;
use teloxide::types::MessageEntityKind::{Mention, TextMention};
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::types::{FileMeta, InputFile, User};
use teloxide::utils::markdown::escape;

async fn file_downloader(bot: &Bot, file_meta: &FileMeta) -> ResponseResult<(String, Vec<u8>)> {
    let file = bot.get_file(&file_meta.id).await?;
    let file_id = file_meta.id.clone();
    let mut file_vec: Vec<u8> = Vec::new();
    bot.download_file(file.path.as_str(), &mut file_vec).await?;
    Ok((file_id, file_vec))
}

// pub async fn get_sender_first_name(bot: &Bot, msg: &Message) -> ResponseResult<Option<String>> {
//     match &msg.from {
//         Some(sender) => Ok(Some(sender.first_name.to_string())),
//         None => {
//             bot.send_message(
//                 msg.chat.id,
//                 escape(&format!("error: user `{user_text}` not found.")),
//             )
//                 .parse_mode(MarkdownV2)
//                 .await?;
//             Ok(None)
//         }
//     }
// }

pub async fn get_sender_profile_photo(
    bot: &Bot,
    msg: &Message,
) -> ResponseResult<Option<(String, Vec<u8>)>> {
    let user = match &msg.from {
        Some(sender) => sender,
        None => {
            bot.send_message(msg.chat.id, "error: msg sender not found.")
                .parse_mode(MarkdownV2)
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
            escape(&format!(
                "error: user `{}` has no profile photo.",
                user.first_name
            )),
        )
        .parse_mode(MarkdownV2)
        .await?;
        Ok(None)
    }
}

pub async fn get_final_photo_list(
    bot: &Bot,
    msg: &Message,
) -> ResponseResult<Option<HashMap<String, Vec<u8>>>> {
    let mut final_photo_list = HashMap::<String, Vec<u8>>::new();

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
                            escape(&format!("error: user `{user_text}` not found.")),
                        )
                        .parse_mode(MarkdownV2)
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
                final_photo_list.insert(id, data);
            } else {
                bot.send_message(
                    msg.chat.id,
                    escape(&format!(
                        "error: user `{}` has no profile photo.",
                        user_text
                    )),
                )
                .parse_mode(MarkdownV2)
                .await?;
                return Ok(None);
            }
        };
    }

    if let Some(replied_msg) = msg.reply_to_message() {
        if let Some(photos) = replied_msg.photo() {
            let replied_photo = photos.last().unwrap();
            let (id, data) = file_downloader(bot, &replied_photo.file).await?;
            final_photo_list.insert(id, data);
        }
        if let Some(animation) = replied_msg.animation() {
            let (id, data) = file_downloader(bot, &animation.file).await?;
            final_photo_list.insert(id, data);
        }
    }

    Ok(Some(final_photo_list))
}

pub async fn send_media(
    bot: &Bot,
    msg: &Message,
    content: Vec<u8>,
    filename: String,
) -> ResponseResult<Message> {
    match infer::get(&content) {
        Some(mime_type) => {
            let extension = mime_type.extension();
            let input_file =
                InputFile::memory(content.clone()).file_name(format!("{filename}.{extension}"));

            match mime_type.matcher_type() {
                infer::MatcherType::Image => match mime_type.mime_type() {
                    "image/gif" | "image/webp" => bot.send_animation(msg.chat.id, input_file).await,
                    _ => bot.send_photo(msg.chat.id, input_file).await,
                },
                infer::MatcherType::Video => bot.send_video(msg.chat.id, input_file).await,
                _ => {
                    bot.send_message(msg.chat.id, "error: File type is not media.")
                        .parse_mode(MarkdownV2)
                        .await
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "error: File type unknown.")
                .parse_mode(MarkdownV2)
                .await
        }
    }
}
