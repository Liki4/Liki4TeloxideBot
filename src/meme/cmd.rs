use {
    super::MemeAction,
    crate::meme::{
        generator::{
            error::Error, types::RenderOptions, CLIENT, MEME_KEYWORD_KEY_MAPPING,
            MEME_KEY_INFO_MAPPING,
        },
        utils::{get_final_photo_list, get_sender_profile_photo, send_media},
    },
    futures::executor::block_on,
    meme_generator::meme::MemeInfo,
    rand::prelude::*,
    std::collections::HashMap,
    teloxide::{
        prelude::*,
        types::{ParseMode, ReplyParameters},
        utils::markdown::escape,
    },
};

pub async fn handler(
    bot: &Bot,
    msg: &Message,
    action: MemeAction,
    args: Vec<String>,
) -> ResponseResult<Message> {
    let error = match action {
        MemeAction::Info => match args.first() {
            Some(keyword) => {
                if let Some(key) = MEME_KEYWORD_KEY_MAPPING.get().unwrap().get(keyword) {
                    let preview = CLIENT.render_preview(key).await;
                    let info = CLIENT.get_info(key);

                    match (preview, info) {
                        (Ok(preview), Ok(meme_info)) => {
                            match serde_json::to_string_pretty(&meme_info) {
                                Ok(meme_info_str) => {
                                    return send_media(
                                        bot,
                                        msg,
                                        preview,
                                        key.to_string(),
                                        Some(format!("```json\n{}\n```", meme_info_str)),
                                    )
                                    .await;
                                }
                                Err(_) => {
                                    Error::MemeFeedback("MemeInfo Serialize failed".to_string())
                                }
                            }
                        }
                        (Err(err), _) => err,
                        (_, Err(err)) => err,
                    }
                } else {
                    Error::NoSuchMeme(keyword.clone())
                }
            }
            None => Error::ArgMismatch,
        },
        MemeAction::List => match CLIENT.render_list().await {
            Ok(meme_list) => {
                return send_media(bot, msg, meme_list, "meme_list".to_string(), None).await;
            }
            Err(e) => e,
        },
        MemeAction::Search => match args.first() {
            Some(keyword) => {
                if let Some(key) = MEME_KEYWORD_KEY_MAPPING.get().unwrap().get(keyword) {
                    return bot
                        .send_message(msg.chat.id, escape(key))
                        .parse_mode(ParseMode::MarkdownV2)
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
                        true => info.params.min_images == 1 && info.params.min_texts == 0,
                        false => info.params.min_images == 0 && info.params.min_texts == 1,
                    })
                    .collect::<Vec<_>>()
            }
            fn render_meme_with_profile_photo(
                key: &str,
                file_id: String,
                file_data: Vec<u8>,
            ) -> Result<Vec<u8>, Error> {
                let mut images = Vec::<(String, Vec<u8>)>::new();
                images.push((file_id.clone(), file_data));
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

            let mut meme: Result<Vec<u8>, Error> =
                Err(Error::MemeFeedback("Get random meme failed".to_string()));
            let mut filename = String::new();
            if let Ok(photo) = get_sender_profile_photo(bot, msg).await {
                let mut rng = rand::rng();

                if let Some(user) = &msg.from {
                    if let Some((file_id, file_data)) = photo {
                        if let Some((&ref key, &ref _info)) =
                            get_filtered_memes(true).choose(&mut rng)
                        {
                            meme = render_meme_with_profile_photo(&key, file_id.clone(), file_data);
                            filename = key.to_string();
                        }
                    } else if let Some((&ref key, &ref _info)) =
                        get_filtered_memes(false).choose(&mut rng)
                    {
                        meme = render_meme_with_username(&key, &user.first_name);
                        filename = key.to_string();
                    }
                }
            }
            match meme {
                Ok(meme) => {
                    return send_media(
                        bot,
                        msg,
                        meme,
                        filename.clone(),
                        Some(format!("`{filename}`")),
                    )
                    .await;
                }
                Err(e) => e,
            }
        }
        MemeAction::Generate => match args.first() {
            Some(keyword) => {
                if let Some(key) = MEME_KEYWORD_KEY_MAPPING.get().unwrap().get(keyword) {
                    match CLIENT.get_info(key) {
                        Ok(info) => {
                            let mut meme: Result<Vec<u8>, Error> =
                                Err(Error::MemeFeedback("Generate meme failed".to_string()));
                            if let (Some(photo_list), text_list) =
                                (get_final_photo_list(bot, msg).await?, args[1..].to_vec())
                            {
                                let final_photo_list = photo_list
                                    .iter()
                                    .map(|(id, data)| (id.to_string(), data.clone()))
                                    .take(info.params.max_images as usize)
                                    .collect::<Vec<(String, Vec<u8>)>>();
                                let final_text_list = text_list
                                    .iter()
                                    .map(|s| s.to_string())
                                    .take(info.params.max_texts as usize)
                                    .collect::<Vec<String>>();
                                let options = RenderOptions {
                                    images: {
                                        if final_photo_list.len() > 0 {
                                            Some(final_photo_list)
                                        } else {
                                            None
                                        }
                                    },
                                    texts: {
                                        if final_text_list.len() > 0 {
                                            Some(final_text_list)
                                        } else {
                                            None
                                        }
                                    },
                                    args: Some(HashMap::from([(
                                        "circle".to_string(),
                                        true.into(),
                                    )])),
                                };
                                meme = CLIENT.render_meme(key, options).await;
                            }
                            match meme {
                                Ok(meme) => {
                                    return send_media(bot, msg, meme, key.to_string(), None).await;
                                }
                                Err(e) => e,
                            }
                        }
                        Err(e) => e,
                    }
                } else {
                    Error::NoSuchMeme(keyword.clone())
                }
            }
            None => Error::ArgMismatch,
        },
    };

    bot.send_message(msg.chat.id, escape(&error.to_string()))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await
}
