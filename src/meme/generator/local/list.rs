use crate::meme::generator::error::Error;
use crate::meme::generator::local::client::MemeClient;
use meme_generator::get_meme_keys;
use meme_generator::tools::{render_meme_list, RenderMemeListParams};

impl MemeClient {
    pub async fn render_list_impl(&self) -> Result<Vec<u8>, Error> {
        let params = RenderMemeListParams::default();
        Ok(render_meme_list(params)?)
    }

    pub async fn get_keys_impl(&self) -> Vec<String> {
        let meme_str = get_meme_keys();
        meme_str
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
    }
}
