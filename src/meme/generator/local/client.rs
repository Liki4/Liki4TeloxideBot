use {
    crate::meme::generator::{error::Error, interface::MemeGeneratorApi, types::RenderOptions},
    async_trait::async_trait,
    meme_generator::meme::MemeInfo,
    std::collections::HashMap,
};

pub struct MemeClient {}

impl MemeClient {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MemeGeneratorApi for MemeClient {
    async fn render_meme(&self, key: &str, options: RenderOptions) -> Result<Vec<u8>, Error> {
        self.render_meme_impl(key, options).await
    }

    async fn get_infos(&self) -> Result<HashMap<String, MemeInfo>, Error> {
        self.get_infos_impl().await
    }

    async fn render_list(&self) -> Result<Vec<u8>, Error> {
        self.render_list_impl().await
    }

    async fn get_keys(&self) -> Result<Vec<String>, Error> {
        Ok(self.get_keys_impl().await)
    }

    async fn render_preview(&self, key: &str) -> Result<Vec<u8>, Error> {
        self.render_preview_impl(key).await
    }
}
