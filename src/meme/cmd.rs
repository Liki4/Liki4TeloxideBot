use {
    super::MemeAction,
    crate::meme::{
        generator::{
            error::Error,
            types::RenderOptions,
            CLIENT,
            MEME_KEYWORD_KEY_MAPPING,
            MEME_KEY_INFO_MAPPING,
        },
        utils::{
            get_final_photo_list,
            rand_num,
            send_media,
        },
    },
    meme_generator::meme::MemeInfo,
    std::collections::HashMap,
    teloxide::{
        prelude::*,
        types::{
            ParseMode,
            ReplyParameters,
        },
        utils::markdown::escape,
    },
};

const HELP_MSG: &str = r#"
**Meme Bot Commands:**

`/meme help` \- Display this help message\.
`/meme list` \- List all available memes\.
`/meme info <key/keyword>` \- Display detailed information about a specific meme\.
`/meme search <keyword>` \- Search for a meme by keyword\.
`/meme random [args]` \- Generate a random meme with optional arguments\.
`/meme generate <key/keyword> [args]` \- Generate a specific meme with optional arguments\.

**Arguments Format:**
`[<text1> <text2>] [<@someone> <@someone_else>]`

· **Texts:** Provide text inputs for the meme\.
· **Photos:** Mention users \(`@someone`\) to include their profile photos in the meme\.

**Additional Notes:**
· The **random** and **generate** commands can use photos by
  · attaching an image to the command message
  · mentioning users \(`@someone`\) in the command message
  · replying to a media group message
· The command sender's **first\_name** and **profile\_photo** are automatically appended to the end of the `texts\_list` and `photos\_list`\.

For more details, visit the [GitHub repository Liki4TeloxideBot](https://github\.com/Liki4/Liki4TeloxideBot)\.
"#;

pub async fn meme_command_handler(
    bot: &Bot, msg: &Message, action: MemeAction, args: Vec<String>,
) -> ResponseResult<Message> {
    let error = match action {
        MemeAction::Help => {
            return bot
                .send_message(msg.chat.id, HELP_MSG)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_parameters(ReplyParameters::new(msg.id))
                .await;
        }
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
            let mut meme: Result<Vec<u8>, Error> =
                Err(Error::MemeFeedback("Get random meme failed".to_string()));
            let mut filename = String::new();
            if let (Some(photo_list), text_list) = (get_final_photo_list(bot, msg).await?, {
                let mut combined_args = args;
                if let Some(user) = &msg.from {
                    combined_args.push(user.first_name.clone());
                }
                combined_args
            }) {
                let filtered_meme: Vec<(&String, &MemeInfo)> = MEME_KEY_INFO_MAPPING
                    .get()
                    .unwrap()
                    .iter()
                    .filter(|(_, info)| {
                        info.params.min_images as usize <= photo_list.len()
                            && info.params.min_texts as usize <= text_list.len()
                    })
                    .collect();

                let (&ref key, &ref info) = filtered_meme[rand_num(filtered_meme.len())];
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
                    args: Some(HashMap::from([("circle".to_string(), true.into())])),
                };
                meme = CLIENT.render_meme(key, options).await;
                filename = key.to_string();
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
                                (get_final_photo_list(bot, msg).await?, {
                                    let mut combined_args = args[1..].to_vec();
                                    if let Some(user) = &msg.from {
                                        combined_args.push(user.first_name.clone());
                                    }
                                    combined_args
                                })
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
