use {
    crate::meme::generator::{error::Error, local::client::MemeClient},
    meme_generator::{get_meme, get_meme_keys, meme::MemeInfo},
    std::collections::HashMap,
};

impl MemeClient {
    pub async fn get_infos_impl(&self) -> Result<HashMap<String, MemeInfo>, Error> {
        let keys = get_meme_keys();
        let mut infos: HashMap<String, MemeInfo> = HashMap::new();
        for key in keys {
            let meme_info = self.get_info_impl(key).await?;
            infos.insert(key.to_string(), meme_info);
        }
        Ok(infos)
    }
    pub(crate) async fn get_info_impl(&self, key: &str) -> Result<MemeInfo, Error> {
        if let Some(meme) = get_meme(&key) {
            Ok(meme.info())
        } else {
            Err(Error::NoSuchMeme(format!("Meme `{key}` not found")))
        }
    }
}
