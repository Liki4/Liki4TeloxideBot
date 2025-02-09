pub mod cmd;
pub mod generator;
mod utils;

use {
    std::{
        error::Error,
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
    teloxide::utils::command::{ParseError, ParseError::IncorrectFormat},
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
