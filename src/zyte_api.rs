use crate::RequestBuilder;
use crate::Response;
use http::Method;
use std::error::Error;
use std::str::FromStr;

#[derive(Clone)]
pub struct ZyteApi {
    pub client: reqwest::Client,
    pub api_key: String,
    pub api_url: String,
}
impl ZyteApi {
    pub fn new(api_key: &str) -> ZyteApi {
        ZyteApi {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            api_url: "https://api.zyte.com/v1/extract".to_string(),
        }
    }
    pub async fn get(&self, url: &str) -> Result<Response, Box<dyn Error>> {
        let url = http::Uri::from_str(url)?;
        Ok(RequestBuilder::new(self.clone(), Method::GET, url)
            .send()
            .await?)
    }
    pub fn post(&self, url: &str) -> Result<RequestBuilder, Box<dyn Error>> {
        let url = http::Uri::from_str(url)?;
        Ok(RequestBuilder::new(self.clone(), Method::POST, url))
    }
}
