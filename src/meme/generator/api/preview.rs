use crate::meme::generator::{api::client::MemeApiClient, error::Error};

impl MemeApiClient {
    pub async fn render_preview_impl(&self, key: &str) -> Result<Vec<u8>, Error> {
        let response = self.get(&format!("/memes/{key}/preview")).await?;
        Ok(response.bytes().await?.to_vec())
    }
}
