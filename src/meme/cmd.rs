use super::MemeAction;
use crate::meme::generator::error::Error;
use crate::meme::generator::types::RenderOptions;
use crate::meme::generator::{CLIENT, MEME_KEYWORD_KEY_MAPPING, MEME_KEY_INFO_MAPPING};
use crate::meme::utils::{get_final_photo_list, get_sender_profile_photo, send_media};
use meme_generator::meme::{MemeInfo, OptionValue};
use rand::prelude::*;
use std::collections::HashMap;
use teloxide::prelude::*;
use teloxide::types::InputFile;
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
                let client = CLIENT.lock().await;
                let preview = client.render_preview(key).await;
                let info = client.get_info(key).await;

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
        MemeAction::List => match CLIENT.lock().await.render_list().await {
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
            let client = CLIENT.lock().await;
            match get_sender_profile_photo(bot, msg).await {
                Ok(profile_photo) => {
                    let mut rng = rand::rng();
                    match profile_photo {
                        Some((file_id, file_data)) => {
                            let mut one_arg_memes = MEME_KEY_INFO_MAPPING
                                .get()
                                .unwrap()
                                .iter()
                                .filter(|(_, info)| {
                                    (info.params.max_images == 1
                                        && info.params.min_images == 1
                                        && info.params.max_texts == 0
                                        && info.params.min_texts == 0)
                                        || (info.params.max_texts == 1
                                        && info.params.min_texts == 1
                                        && info.params.max_images == 0
                                        && info.params.min_images == 0)
                                })
                                .collect::<Vec<_>>();
                            one_arg_memes.shuffle(&mut rng);
                            match one_arg_memes.choose(&mut rng) {
                                Some(&(key, info)) => {
                                    match (info.params.max_images, info.params.max_texts) {
                                        (_, 1) => {
                                            let mut images = HashMap::<String, Vec<u8>>::new();
                                            images.insert(file_id.clone(), file_data);
                                            let mut args = HashMap::<String, OptionValue>::new();
                                            args.insert("circle".to_string(), true.into());
                                            let options = RenderOptions {
                                                images: Some(images),
                                                texts: None,
                                                args: Some(args),
                                            };
                                            // match client.render_meme(key.as_str(), options)
                                            //     .await
                                            // {
                                            //     Ok(meme) => {
                                            //         return send_media(bot, msg, meme, file_id)
                                            //             .await;
                                            //     }
                                            //     Err(e) => e,
                                            // }
                                            Error::MemeFeedback("TEST_ERROR".to_string())
                                        }
                                        (1, _) => match &msg.from {
                                            Some(user) => {
                                                let username = user.first_name.clone();
                                                let mut args =
                                                    HashMap::<String, OptionValue>::new();
                                                args.insert("circle".to_string(), true.into());
                                                let options = RenderOptions {
                                                    images: None,
                                                    texts: Some(vec![username]),
                                                    args: Some(args),
                                                };
                                                // match client.render_meme(key.as_str(), options)
                                                //     .await
                                                // {
                                                //     Ok(meme) => {
                                                //         return send_media(bot, msg, meme, file_id)
                                                //             .await;
                                                //     }
                                                //     Err(e) => e,
                                                // }
                                                Error::MemeFeedback("TEST_ERROR".to_string())
                                            }
                                            None => Error::ParamsMismatch,
                                        },
                                        (_, _) => Error::ParamsMismatch,
                                    }
                                }
                                None => Error::MemeFeedback("Get random meme failed.".to_string()),
                            }
                        }
                        None => {
                            let mut one_arg_memes = MEME_KEY_INFO_MAPPING
                                .get()
                                .unwrap()
                                .iter()
                                .filter(|(_, info)| {
                                    info.params.max_texts == 1
                                        && info.params.min_texts == 1
                                        && info.params.max_images == 0
                                        && info.params.min_images == 0
                                })
                                .collect::<Vec<_>>();
                            one_arg_memes.shuffle(&mut rng);
                            match one_arg_memes.choose(&mut rng) {
                                Some(&(key, info)) => match &msg.from {
                                    Some(user) => {
                                        let username = user.first_name.clone();
                                        let mut args = HashMap::<String, OptionValue>::new();
                                        args.insert("circle".to_string(), true.into());
                                        let options = RenderOptions {
                                            images: None,
                                            texts: Some(vec![username.clone()]),
                                            args: Some(args),
                                        };
                                        // match client.render_meme(key.as_str(), options)
                                        //     .await
                                        // {
                                        //     Ok(meme) => {
                                        //         return send_media(bot, msg, meme, username).await;
                                        //     }
                                        //     Err(e) => e,
                                        // }
                                        Error::MemeFeedback("TEST_ERROR.".to_string())
                                    }
                                    None => Error::ParamsMismatch,
                                },
                                None => Error::MemeFeedback("Get random meme failed.".to_string()),
                            }
                        }
                    }
                }
                Err(e) => Error::MemeFeedback(e.to_string()),
            }
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
