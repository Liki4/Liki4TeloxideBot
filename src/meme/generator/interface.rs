use super::{error::Error, MEME_KEY_INFO_MAPPING};
use crate::meme::generator::types::RenderOptions;
use async_trait::async_trait;
use meme_generator::meme::MemeInfo;
use std::collections::HashMap;

#[async_trait]
pub trait MemeGeneratorApi {
    async fn render_meme(&self, key: &str, options: RenderOptions) -> Result<Vec<u8>, Error>;
    async fn get_infos(&self) -> Result<HashMap<String, MemeInfo>, Error>;
    fn get_info(&self, key: &str) -> Result<MemeInfo, Error> {
        MEME_KEY_INFO_MAPPING
            .get()
            .ok_or(Error::MemeFeedback("not found meme mapping".to_string()))?
            .get(key)
            .cloned()
            .ok_or(Error::NoSuchMeme(key.to_string()))
    }
    async fn render_list(&self) -> Result<Vec<u8>, Error>;
    async fn get_keys(&self) -> Result<Vec<String>, Error>;
    async fn render_preview(&self, key: &str) -> Result<Vec<u8>, Error>;
}
