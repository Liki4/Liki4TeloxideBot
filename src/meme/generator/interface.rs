use super::error::Error;
use crate::meme::generator::types::RenderOptions;
use async_trait::async_trait;
use meme_generator::meme::MemeInfo;
use std::collections::HashMap;

#[async_trait]
pub trait MemeGeneratorApi {
    async fn render_meme(&self, key: &str, options: RenderOptions) -> Result<Vec<u8>, Error>;
    async fn get_infos(&self) -> Result<HashMap<String, MemeInfo>, Error>;
    async fn get_info(&self, key: &str) -> Result<MemeInfo, Error>;
    async fn render_list(&self) -> Result<Vec<u8>, Error>;
    async fn get_keys(&self) -> Result<Vec<String>, Error>;
    async fn render_preview(&self, key: &str) -> Result<Vec<u8>, Error>;
}
