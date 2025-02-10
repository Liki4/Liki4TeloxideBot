use {
    crate::meme::generator::{
        error::Error,
        local::client::MemeClient,
    },
    meme_generator::get_meme,
    std::collections::HashMap,
};

impl MemeClient {
    pub async fn render_preview_impl(&self, key: &str) -> Result<Vec<u8>, Error> {
        let meme = match get_meme(&key) {
            Some(meme) => meme,
            None => return Err(Error::NoSuchMeme(format!("Meme `{key}` not found"))),
        };
        Ok(meme.generate_preview(HashMap::from([("circle".to_string(), true.into())]))?)
    }
}
