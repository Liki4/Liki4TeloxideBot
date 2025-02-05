use crate::meme::generator::api::client::{MemeApiClient, PostBody};
use crate::meme::generator::error::Error;
use crate::meme::generator::types::RenderOptions;
use reqwest::multipart::{Form, Part};

impl MemeApiClient {
    pub async fn render_meme_impl(
        &self,
        key: &str,
        options: RenderOptions,
    ) -> Result<Vec<u8>, Error> {
        let mut form = Form::new();
        if let Some(images) = options.images {
            for (file_id, file_content) in images {
                let mime_type = infer::get(&file_content).expect("file type is unknown");
                let extension = mime_type.extension();
                let file_part = Part::bytes(file_content)
                    .file_name(format!("{file_id}.{extension}"))
                    .mime_str(mime_type.mime_type())?;
                form = form.part("images", file_part);
            }
        };
        if let Some(texts) = options.texts {
            for text in texts {
                form = form.text("texts", serde_json::to_string(&text).unwrap());
            }
        };
        if let Some(args) = options.args {
            form = form.text("args", serde_json::to_string(&args).unwrap());
        }
        let response = self
            .post(&format!("/memes/{key}/"), PostBody::Multipart(form))
            .await?;
        Ok(response.bytes().await?.to_vec())
    }
}
