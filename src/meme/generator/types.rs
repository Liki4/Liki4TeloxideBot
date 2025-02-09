// #region alconna

use meme_generator::meme::OptionValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 节点触发的动作类型
/// @see https://github.com/ArcletProject/Alconna/blob/75196c3/src/arclet/alconna/action.py#L6
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub enum ActType {
//     /// 无 Args 时, 仅存储一个值, 默认为 Ellipsis; 有 Args 时, 后续的解析结果会覆盖之前的值
//     STORE = 0,
//     /// 无 Args 时, 将多个值存为列表, 默认为 Ellipsis; 有 Args 时, 每个解析结果会追加到列表中
//     ///
//     /// 当存在默认值并且不为列表时, 会自动将默认值变成列表, 以保证追加的正确性
//     APPEND = 1,
//     /// 无 Args 时, 计数器加一; 有 Args 时, 表现与 STORE 相同
//     ///
//     /// 当存在默认值并且不为数字时, 会自动将默认值变成 1, 以保证计数器的正确性
//     COUNT = 2,
// }

/// 节点触发的动作
/// @see https://github.com/ArcletProject/Alconna/blob/75196c3/src/arclet/alconna/action.py#L24
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Action {
//     pub act_type: ActType,
//     pub value: Option<serde_json::Value>, // 使用 serde_json::Value 来表示任意类型的值
// }

/// 标识参数单元的特殊属性
/// @see https://github.com/ArcletProject/Alconna/blob/75196c3/src/arclet/alconna/args.py#L28
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub enum ArgFlag {
//     #[serde(rename = "?")]
//     OPTIONAL,
//     #[serde(rename = "/")]
//     HIDDEN,
//     #[serde(rename = "!")]
//     ANTI,
// }

// impl Serialize for ArgFlag {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer
//     {
//         serializer.serialize_str(match *self {
//             ArgFlag::OPTIONAL => "?",
//             ArgFlag::HIDDEN => "/",
//             ArgFlag::ANTI => "!",
//         })
//     }
// }
//
// impl<'de> Deserialize<'de> for ArgFlag {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where D: Deserializer<'de>
//     {
//         let s = String::deserialize(deserializer)?;
//         Ok(match s.as_str() {
//             "?" => ArgFlag::OPTIONAL,
//             "/" => ArgFlag::HIDDEN,
//             "!" => ArgFlag::ANTI,
//             _ => ArgFlag::OPTIONAL,
//         })
//     }
// }

// #endregion

// #region meme-generator

/// @see https://github.com/MeetWq/meme-generator/blob/8bb9fba/meme_generator/meme.py#L23
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "lowercase")]
// pub enum UserInfoGender {
//     Male,
//     Female,
//     Unknown,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UserInfo {
//     pub name: Option<String>,
//     pub gender: Option<UserInfoGender>,
// }

/// @see https://github.com/MeetWq/meme-generator/blob/8bb9fba/meme_generator/meme.py#L44
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ParserArg {
//     pub name: String,
//     pub value: String,
//     pub default: Option<serde_json::Value>,
//     pub flags: Option<Vec<ArgFlag>>,
// }

/// @see https://github.com/MeetWq/meme-generator/blob/8bb9fba/meme_generator/meme.py#L51
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ParserOption {
//     pub names: Vec<String>,
//     pub args: Option<Vec<ParserArg>>,
//     pub dest: Option<String>,
//     pub default: Option<serde_json::Value>,
//     pub action: Option<Action>,
//     pub help_text: Option<String>,
//     pub compact: Option<bool>,
// }

/// @see https://github.com/MeetWq/meme-generator/blob/8bb9fba/meme_generator/meme.py#L81
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CommandShortcut {
//     pub key: String,
//     pub args: Option<Vec<String>>,
//     pub humanized: Option<String>,
// }

/// @see https://github.com/MeetWq/meme-generator/blob/8bb9fba/meme_generator/app.py#L24
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemeArgsResponse {
//     pub args_model: serde_json::Value,
//     pub args_examples: Vec<serde_json::Value>,
//     pub parser_options: Vec<ParserOption>,
// }

/// @see https://github.com/MeetWq/meme-generator/blob/8bb9fba/meme_generator/app.py#L30
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemeParamsResponse {
//     pub min_images: i32,
//     pub max_images: i32,
//     pub min_texts: i32,
//     pub max_texts: i32,
//     pub default_texts: Vec<String>,
//     pub args_type: Option<MemeArgsResponse>,
// }

/// @see https://github.com/MeetWq/meme-generator/blob/8bb9fba/meme_generator/app.py#L39
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemeInfoResponse {
//     pub key: String,
//     pub params_type: MemeParamsResponse,
//     pub keywords: Vec<String>,
//     pub shortcuts: Vec<CommandShortcut>,
//     pub tags: Vec<String>,
//     pub date_created: String,
//     pub date_modified: String,
// }

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

/// @see https://github.com/pydantic/pydantic-core/blob/4113638/python/pydantic_core/__init__.py#L73
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PyDanticErrorDetails {
//     pub type_: String,
//     pub loc: Vec<String>,
//     pub msg: String,
//     pub input: serde_json::Value,
//     pub ctx: Option<serde_json::Value>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemeErrorResponse {
//     pub detail: Result<Vec<PyDanticErrorDetails>, String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum OptionValue {
//     Boolean(bool),
//     String(String),
//     Integer(i32),
//     Float(f32),
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderOptions {
    pub images: Option<Vec<(String, Vec<u8>)>>,
    pub texts: Option<Vec<String>>,
    pub args: Option<HashMap<String, OptionValue>>,
}

// #endregion
