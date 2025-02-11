use {
    crate::meme::generator::{
        error::{
            response_handler,
            Error,
        },
        interface::MemeGeneratorApi,
        types::RenderOptions,
    },
    async_trait::async_trait,
    meme_generator::meme::MemeInfo,
    reqwest::{
        multipart::Form,
        Client,
        Response,
    },
    serde_json::Value,
    std::collections::HashMap,
    url::Url,
};

pub struct MemeApiClient {
    base_url: Url,
    client: Client,
}

pub enum PostBody {
    Json(Value),
    Multipart(Form),
}

impl MemeApiClient {
    pub fn new(base_url: &str) -> Result<Self, Error> {
        let client = Client::builder().build()?;
        Ok(Self {
            base_url: Url::parse(&base_url)?,
            client,
        })
    }

    pub async fn get(&self, path: &str) -> Result<Response, Error> {
        let url = self.base_url.join(path)?;
        log::debug!("Requesting meme GET at: {}", url);
        let response = self.client.get(url).send().await?;
        response_handler(response).await
    }

    pub async fn post(&self, path: &str, body: PostBody) -> Result<Response, Error> {
        let url = self.base_url.join(path)?;
        log::debug!("Requesting meme POST at: {}", url);
        let request_builder = self.client.post(url);
        let request = match body {
            PostBody::Json(data) => request_builder.json(&data),
            PostBody::Multipart(form) => request_builder.multipart(form),
        };
        let response = request.send().await?;

        response_handler(response).await
    }
}

#[async_trait]
impl MemeGeneratorApi for MemeApiClient {
    async fn render_meme(&self, key: &str, options: RenderOptions) -> Result<Vec<u8>, Error> {
        self.render_meme_impl(key, options).await
    }

    async fn get_infos(&self) -> Result<HashMap<String, MemeInfo>, Error> {
        self.get_infos_impl().await
    }

    async fn render_list(&self) -> Result<Vec<u8>, Error> {
        self.render_list_impl().await
    }

    async fn render_preview(&self, key: &str) -> Result<Vec<u8>, Error> {
        self.render_preview_impl(key).await
    }
}
