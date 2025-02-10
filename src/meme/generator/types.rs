use {
    meme_generator::meme::OptionValue,
    serde::{
        Deserialize,
        Serialize,
    },
    std::collections::HashMap,
};

/// @see https://github.com/MeetWq/meme-generator/blob/8bb9fba/meme_generator/app.py#L93
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemeKeyWithPropertiesLabel {
    New,
    Hot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeKeyWithProperties {
    pub meme_key: String,
    pub disabled: Option<bool>,
    pub labels: Option<Vec<MemeKeyWithPropertiesLabel>>,
}

/// @see https://github.com/MeetWq/meme-generator/blob/8bb9fba/meme_generator/app.py#L105
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RenderMemeListRequest {
    pub meme_list: Option<Vec<MemeKeyWithProperties>>,
    pub text_template: Option<String>,
    pub add_category_icon: Option<bool>,
}

impl Default for RenderMemeListRequest {
    fn default() -> Self {
        Self {
            meme_list: Some(Vec::<MemeKeyWithProperties>::new()),
            text_template: Some("{index}. {keywords}".to_string()),
            add_category_icon: Some(true),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderOptions {
    pub images: Option<Vec<(String, Vec<u8>)>>,
    pub texts: Option<Vec<String>>,
    pub args: Option<HashMap<String, OptionValue>>,
}
