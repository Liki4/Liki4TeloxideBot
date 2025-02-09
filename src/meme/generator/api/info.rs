use {
    crate::meme::generator::{api::client::MemeApiClient, error::Error},
    meme_generator::meme::MemeInfo,
    std::collections::HashMap,
};

impl MemeApiClient {
    pub async fn get_infos_impl(&self) -> Result<HashMap<String, MemeInfo>, Error> {
        let keys = self.get_keys_impl().await?;
        let mut infos: HashMap<String, MemeInfo> = HashMap::new();
        for key in keys {
            let meme_info = self.get_info_impl(&key).await?;
            infos.insert(key, meme_info);
        }
        Ok(infos)
    }
    pub async fn get_info_impl(&self, key: &str) -> Result<MemeInfo, Error> {
        let response = self.get(&format!("/memes/{key}/info")).await?;
        Ok(response.json().await?)
    }
}
