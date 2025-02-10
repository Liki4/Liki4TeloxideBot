use {
    reqwest::Response,
    serde::{
        Deserialize,
        Serialize,
    },
    std::fmt::Display,
};

#[derive(Debug, Serialize)]
pub enum Error {
    ReqwestError(String),

    // local
    ImageDecodeError(String),
    ImageEncodeError(String),
    ImageAssetMissing(String),
    DeserializeError(String),
    // local

    // api
    MemeGeneratorException(String),
    NoSuchMeme(String),
    OpenImageFailed(String),
    ParamsMismatch,
    TextOrNameNotEnough(String),
    ArgMismatch,
    ArgParserMismatch(String),
    ArgModelMismatch(String),
    // api

    // both
    ImageNumberMismatch(u8, u8, u8),
    TextNumberMismatch(u8, u8, u8),
    TextOverLength(String),

    MemeFeedback(String),
    //both
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e.to_string())
    }
}

impl From<meme_generator::error::Error> for Error {
    fn from(e: meme_generator::error::Error) -> Self {
        match e {
            meme_generator::error::Error::ImageDecodeError(msg) => Error::ImageDecodeError(msg),
            meme_generator::error::Error::ImageEncodeError(msg) => Error::ImageEncodeError(msg),
            meme_generator::error::Error::ImageAssetMissing(msg) => Error::ImageAssetMissing(msg),
            meme_generator::error::Error::DeserializeError(msg) => Error::DeserializeError(msg),
            meme_generator::error::Error::ImageNumberMismatch(min, max, actual) => {
                Error::ImageNumberMismatch(min, max, actual)
            }
            meme_generator::error::Error::TextNumberMismatch(min, max, actual) => {
                Error::TextNumberMismatch(min, max, actual)
            }
            meme_generator::error::Error::TextOverLength(msg) => Error::TextOverLength(msg),
            meme_generator::error::Error::MemeFeedback(msg) => Error::MemeFeedback(msg),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[derive(Deserialize)]
pub struct MemeGeneratorApiResponseError {
    detail: String,
}

pub async fn response_handler(resp: Response) -> Result<Response, Error> {
    let status = resp.status();
    if status.is_success() {
        return Ok(resp);
    }
    let detail = resp.json::<MemeGeneratorApiResponseError>().await?.detail;
    match status.as_u16() {
        520 => Err(Error::MemeGeneratorException(detail)),
        531 => Err(Error::NoSuchMeme(detail)),
        532 => Err(Error::TextOverLength(detail)),
        533 => Err(Error::OpenImageFailed(detail)),
        540 => Err(Error::ParamsMismatch),
        541 => {
            let numbers = detail
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse::<u8>().ok())
                .collect::<Vec<u8>>(); // 图片数量应为 (\d+)(?: ~ (\d+))?
            Err(Error::ImageNumberMismatch(
                numbers.get(0).unwrap_or(&255).clone(),
                numbers.get(1).unwrap_or(&255).clone(),
                255,
            ))
        }
        542 => {
            let numbers = detail
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse::<u8>().ok())
                .collect::<Vec<u8>>(); // 文本数量不符，文本数量应为 (\d+)(?: ~ (\d+))?
            Err(Error::TextNumberMismatch(
                numbers.get(0).unwrap_or(&255).clone(),
                numbers.get(1).unwrap_or(&255).clone(),
                255,
            ))
        }
        543 => Err(Error::TextOrNameNotEnough(detail)),
        550 => Err(Error::ArgMismatch),
        551 => Err(Error::ArgParserMismatch(detail)),
        552 => Err(Error::ArgModelMismatch(detail)),
        560 => Err(Error::MemeFeedback(detail)),
        _ => Err(Error::ReqwestError(status.to_string())),
    }
}
