use crate::meme::generator::{
    api::client::{MemeApiClient, PostBody},
    error::Error,
    types::RenderMemeListRequest,
};

impl MemeApiClient {
    pub async fn render_list_impl(&self) -> Result<Vec<u8>, Error> {
        let data = RenderMemeListRequest::default();
        let response = self
            .post(
                "/memes/render_list",
                PostBody::Json(serde_json::to_value(&data).unwrap()),
            )
            .await?;
        Ok(response.bytes().await?.to_vec())
    }

    pub async fn get_keys_impl(&self) -> Result<Vec<String>, Error> {
        let response = self.get("/memes/keys").await?;
        Ok(response.json::<Vec<String>>().await?)
    }
}
