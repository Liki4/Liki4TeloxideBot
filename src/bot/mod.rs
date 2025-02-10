use crate::meme::generator::{
    init_meme_mapping,
    init_resources,
};

pub mod handler;
pub mod util;

pub async fn bot_init() {
    init_resources().await;
    init_meme_mapping().await;
}
