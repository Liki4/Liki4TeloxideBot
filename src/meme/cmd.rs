use super::MemeAction;
use crate::meme::generator::error::Error;
use crate::meme::generator::types::RenderOptions;
use crate::meme::generator::{CLIENT, MEME_KEYWORD_KEY_MAPPING, MEME_KEY_INFO_MAPPING};
use crate::meme::utils::{get_final_photo_list, get_sender_profile_photo, send_media};
use futures::executor::block_on;
use meme_generator::meme::{MemeInfo, OptionValue};
use rand::prelude::*;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use teloxide::prelude::*;
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::utils::markdown::escape;

pub async fn handler(
    bot: &Bot,
    msg: &Message,
    action: MemeAction,
    args: Vec<String>,
) -> ResponseResult<Message> {
    let error = match action {
        MemeAction::Info => match args.first() {
            Some(key) => {
                let preview = CLIENT.render_preview(key).await;
                let info = CLIENT.get_info(key);

                match (preview, info) {
                    (Ok(preview), Ok(meme_info)) => {
                        send_media(bot, msg, preview, key.to_string()).await?;

                        let size = size_of_val(&meme_info);
                        log::info!("{}", size);

                        match serde_json::to_string_pretty(&meme_info) {
                            Ok(meme_info_str) => {
                                return bot
                                    .send_message(
                                        msg.chat.id,
                                        escape(&format!("```json\n{}\n```", meme_info_str)),
                                    )
                                    .parse_mode(MarkdownV2)
                                    .await;
                            }
                            Err(_) => Error::MemeFeedback("MemeInfo Serialize failed.".to_string()),
                        }
                    }
                    (Err(err), _) => err,
                    (_, Err(err)) => err,
                }
            }
            None => Error::ArgMismatch,
        },
        MemeAction::List => match CLIENT.render_list().await {
            Ok(meme_list) => {
                return send_media(bot, msg, meme_list, "meme_list".to_string()).await;
            }
            Err(e) => e,
        },
        MemeAction::Search => match args.first() {
            Some(keyword) => {
                if let Some(key) = MEME_KEYWORD_KEY_MAPPING.get().unwrap().get(keyword) {
                    return bot
                        .send_message(msg.chat.id, escape(key))
                        .parse_mode(MarkdownV2)
                        .await;
                }
                Error::NoSuchMeme(keyword.clone())
            }
            None => Error::ArgMismatch,
        },
        MemeAction::Random => {
            // functions begin
            fn get_filtered_memes(
                with_profile_photo: bool,
            ) -> Vec<(&'static String, &'static MemeInfo)> {
                MEME_KEY_INFO_MAPPING
                    .get()
                    .unwrap()
                    .iter()
                    .filter(|(_, info)| match with_profile_photo {
                        true => info.params.min_images <= 1 && info.params.min_texts <= 1,
                        false => info.params.min_images == 0 && info.params.min_texts <= 1,
                    })
                    .collect::<Vec<_>>()
            }
            fn render_meme_with_profile_photo(
                key: &str,
                file_id: String,
                file_data: Vec<u8>,
            ) -> Result<Vec<u8>, Error> {
                let mut images = HashMap::<String, Vec<u8>>::new();
                images.insert(file_id.clone(), file_data);
                let options = RenderOptions {
                    images: Some(images),
                    texts: None,
                    args: Some(HashMap::from([("circle".to_string(), true.into())])),
                };
                block_on(CLIENT.render_meme(key, options))
            }
            fn render_meme_with_username(key: &str, username: &str) -> Result<Vec<u8>, Error> {
                let options = RenderOptions {
                    images: None,
                    texts: Some(vec![username.to_string()]),
                    args: Some(HashMap::from([("circle".to_string(), true.into())])),
                };

                block_on(CLIENT.render_meme(key, options))
            }
            // functions end

            if let Ok(photo) = get_sender_profile_photo(bot, msg).await {
                let mut rng = rand::rng();

                if let Some((file_id, file_data)) = photo {
                    if let Some((&ref key, &ref info)) = get_filtered_memes(true).choose(&mut rng) {
                        if let Ok(meme) =
                            render_meme_with_profile_photo(&key, file_id.clone(), file_data)
                        {
                            return block_on(send_media(bot, msg, meme, file_id));
                        }
                    }
                } else if let Some(user) = &msg.from {
                    if let Some((&ref key, &ref info)) = get_filtered_memes(false).choose(&mut rng)
                    {
                        let username = &user.first_name;
                        if let Ok(meme) = render_meme_with_username(&key, &user.first_name) {
                            return block_on(send_media(bot, msg, meme, username.to_string()));
                        }
                    }
                }
            }
            Error::MemeFeedback("Get random meme failed.".to_string())

            // todo!()
        }
        MemeAction::Generate => {
            let final_photo_list = get_final_photo_list(bot, msg).await?;
            todo!()
        }
    };

    bot.send_message(msg.chat.id, escape(&error.to_string()))
        .parse_mode(MarkdownV2)
        .await
}
