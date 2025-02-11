use {
    crate::meme::generator::{
        api::client::MemeApiClient,
        error::Error,
    },
    chrono::{
        DateTime,
        Local,
        NaiveDateTime,
        TimeZone,
    },
    indicatif::{
        ProgressBar,
        ProgressStyle,
    },
    meme_generator::meme::{
        MemeInfo,
        MemeOption,
        MemeParams,
        MemeShortcut,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    serde_json::Value,
    std::collections::{
        HashMap,
        HashSet,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeParamsWrapper {
    pub min_images: u8,
    pub max_images: u8,
    pub min_texts: u8,
    pub max_texts: u8,
    pub default_texts: Vec<String>,
    pub args_type: Value,
}

impl From<MemeParamsWrapper> for MemeParams {
    fn from(d: MemeParamsWrapper) -> MemeParams {
        MemeParams {
            min_images: d.min_images,
            max_images: d.max_images,
            min_texts: d.min_texts,
            max_texts: d.max_texts,
            default_texts: d.default_texts,
            options: {
                let mut options = Vec::new();
                if let Some(po) = d.args_type.get("parser_options") {
                    options.push(MemeOption::String {
                        name: "options_raw".to_string(),
                        default: Some(po.to_string()),
                        choices: None,
                        description: Some(
                            "warning: parse option from python api is not support for now"
                                .to_string(),
                        ),
                        parser_flags: Default::default(),
                    })
                }
                options
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeInfoWrapper {
    pub key: String,
    pub params_type: MemeParamsWrapper,
    pub keywords: Vec<String>,
    pub shortcuts: Vec<Value>,
    pub tags: HashSet<String>,
    pub date_created: String,
    pub date_modified: String,
}

impl From<MemeInfoWrapper> for MemeInfo {
    fn from(d: MemeInfoWrapper) -> MemeInfo {
        MemeInfo {
            key: d.key,
            params: d.params_type.into(),
            keywords: d.keywords,
            shortcuts: {
                let mut s = Vec::<MemeShortcut>::new();
                for v in d.shortcuts {
                    s.push(MemeShortcut {
                        pattern: v
                            .get("key")
                            .map(|v| v.to_string())
                            .map_or("".to_string(), |v| v.to_string()),
                        humanized: v.get("humanized").map(|v| v.to_string()),
                        names: vec![],
                        texts: vec![],
                        options: HashMap::new(),
                    })
                }
                s
            },
            tags: d.tags,
            date_created: {
                let naive_date_time_result = d.date_created.parse::<NaiveDateTime>();
                if let Ok(naive_date_time) = naive_date_time_result {
                    let local_date_time: DateTime<Local> =
                        Local.from_utc_datetime(&naive_date_time);
                    local_date_time
                } else {
                    Local::now()
                }
            },
            date_modified: {
                let naive_date_time_result = d.date_modified.parse::<NaiveDateTime>();
                if let Ok(naive_date_time) = naive_date_time_result {
                    let local_date_time: DateTime<Local> =
                        Local.from_utc_datetime(&naive_date_time);
                    local_date_time
                } else {
                    Local::now()
                }
            },
        }
    }
}

impl MemeApiClient {
    pub async fn get_infos_impl(&self) -> Result<HashMap<String, MemeInfo>, Error> {
        let keys = self.get_keys().await?;
        let mut infos: HashMap<String, MemeInfo> = HashMap::new();
        let bar = ProgressBar::new(keys.len() as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        for key in keys {
            let meme_info = self.get_info_impl(&key).await?;
            infos.insert(key, meme_info);
            bar.inc(1);
        }
        bar.finish();
        Ok(infos)
    }
    pub async fn get_info_impl(&self, key: &str) -> Result<MemeInfo, Error> {
        let response = self.get(&format!("/memes/{key}/info")).await?;
        let wrapper: MemeInfoWrapper = response.json().await?;
        Ok(wrapper.into())
    }
}
