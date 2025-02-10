use {
    crate::meme::{
        generator::{
            error::Error,
            local::client::MemeClient,
            types::RenderOptions,
        },
        utils::hash_short,
    },
    meme_generator::{
        get_meme,
        meme,
    },
    std::collections::HashMap,
};

impl MemeClient {
    pub async fn render_meme_impl(
        &self, key: &str, options: RenderOptions,
    ) -> Result<Vec<u8>, Error> {
        let meme = match get_meme(key) {
            Some(meme) => meme,
            None => return Err(Error::NoSuchMeme(format!("Meme `{key}` not found"))),
        };

        let id_to_data = options.images.unwrap_or(Vec::new());

        let mut images: Vec<meme::Image> = Vec::new();
        id_to_data.iter().for_each(|(name, data)| {
            images.push(meme::Image {
                name: hash_short(&name).to_string(),
                data: data.clone(),
            })
        });
        let texts = options.texts.unwrap_or(Vec::new());
        let options = options.args.unwrap_or(HashMap::new());

        Ok(meme.generate(images, texts, options)?)
    }
}
