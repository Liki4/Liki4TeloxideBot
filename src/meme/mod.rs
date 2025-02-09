pub mod cmd;
pub mod generator;
mod utils;

use {
    lazy_static::lazy_static,
    std::{
        collections::HashMap,
        error::Error,
        fmt::{self, Display, Formatter},
        str::FromStr,
        sync::{Arc, Mutex},
        time::Duration,
    },
    teloxide::utils::command::{ParseError, ParseError::IncorrectFormat},
    tokio::time::sleep,
};

#[derive(Debug, PartialEq, Clone)]
pub enum MemeAction {
    List,
    Search,
    Info,
    Random,
    Generate,
}

impl FromStr for MemeAction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "info" => Ok(MemeAction::Info),
            "list" => Ok(MemeAction::List),
            "search" => Ok(MemeAction::Search),
            "random" => Ok(MemeAction::Random),
            "generate" => Ok(MemeAction::Generate),
            _ => Err(IncorrectFormat(
                Box::<dyn Error + Send + Sync + 'static>::from("Unknown MemeAction"),
            )),
        }
    }
}

impl Display for MemeAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MemeAction::Info => write!(f, "info"),
            MemeAction::List => write!(f, "list"),
            MemeAction::Search => write!(f, "search"),
            MemeAction::Random => write!(f, "random"),
            MemeAction::Generate => write!(f, "generate"),
        }
    }
}

pub fn parser(input: String) -> Result<(MemeAction, Vec<String>), ParseError> {
    let mut sv = input.trim().split_whitespace();
    let action = MemeAction::from_str(sv.next().unwrap_or_default())?;
    let args: Vec<String> = sv.map(|s| s.to_string()).collect();
    Ok((action, args))
}

pub struct ExpiringHashMap {
    map: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl ExpiringHashMap {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new())),
        }
    }

    pub fn push_value(&self, key: &str, value: String, ttl: Duration) {
        let mut guard = self.map.lock().unwrap();
        let entry = guard.entry(key.to_string()).or_insert(Vec::new());
        entry.push(value.clone());

        if entry.len() == 1 {
            let map_clone = Arc::clone(&self.map);
            let key = key.to_string();
            tokio::spawn(async move {
                sleep(ttl).await;
                let mut guard = map_clone.lock().unwrap();
                guard.remove(&key);
            });
        }
    }

    pub fn get_values(&self, key: &str) -> Option<Vec<String>> {
        let guard = self.map.lock().unwrap();
        guard.get(key).cloned()
    }
}

lazy_static! {
    pub static ref MEDIA_GROUP_MAPPING: ExpiringHashMap = ExpiringHashMap::new();
}
