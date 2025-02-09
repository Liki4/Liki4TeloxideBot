pub mod api;
pub mod error;
pub mod interface;
pub mod local;
pub mod types;

use {
    super::generator::{
        api::client::MemeApiClient, interface::MemeGeneratorApi, local::client::MemeClient,
    },
    lazy_static::lazy_static,
    meme_generator::{meme::MemeInfo, resources::check_resources},
    std::{collections::HashMap, env, sync::Arc},
    tokio::sync::OnceCell,
};

lazy_static! {
    pub static ref CLIENT: Arc<dyn MemeGeneratorApi + Send + Sync> = match env::var("MEME_API_URL")
    {
        Ok(url) => Arc::new(MemeApiClient::new(&url).unwrap()),
        Err(_) => Arc::new(MemeClient::new()),
    };
}

pub async fn init_resources() {
    log::info!("Initializing resources...");
    if env::var("MEME_API_URL").is_err() {
        check_resources(None).await;
    };
    log::info!("Resources initialized!");
}

pub static MEME_KEYWORD_KEY_MAPPING: OnceCell<HashMap<String, String>> = OnceCell::const_new();
pub static MEME_KEY_INFO_MAPPING: OnceCell<HashMap<String, MemeInfo>> = OnceCell::const_new();
pub async fn init_meme_mapping() {
    log::info!("Initializing meme mapping...");

    let infos = CLIENT.get_infos().await.unwrap();
    let mut meme_keyword_key_map = HashMap::<String, String>::new();
    let mut meme_key_info_map = HashMap::<String, MemeInfo>::new();

    for (key, meme_info) in infos {
        meme_keyword_key_map.insert(key.clone(), key.clone());
        meme_key_info_map.insert(key.clone(), meme_info.clone());

        for keyword in meme_info.keywords {
            if key != keyword {
                meme_keyword_key_map.insert(keyword, key.clone());
            }
        }
    }
    MEME_KEYWORD_KEY_MAPPING
        .get_or_init(|| async {
            log::info!("Keyword mapping: {}", meme_keyword_key_map.len());
            meme_keyword_key_map
        })
        .await;
    MEME_KEY_INFO_MAPPING
        .get_or_init(|| async {
            log::info!("Info mapping: {}", meme_key_info_map.len());
            meme_key_info_map
        })
        .await;
    log::info!("Meme mapping initialized!");
}
