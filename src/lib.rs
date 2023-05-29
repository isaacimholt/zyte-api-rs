//! # zyte-api-rs
//!
//! This is an unofficial rust package for the Zyte API.
pub mod request;
pub mod response;

use base64::{engine::general_purpose, Engine as _};
use http::Method;
use std::error::Error;
use std::str::FromStr;
use request::{Request, HttpRequestBodyType};
use response::Response;

pub struct RequestBuilder {
    client: ZyteApi,
    request: Request,
}

impl RequestBuilder {
    pub fn new(client: ZyteApi, method: Method, url: http::Uri) -> RequestBuilder {
        RequestBuilder {
            client,
            request: Request {
                url,
                http_response_body: Some(true),
                http_request_method: Some(method),
                ..Default::default()
            },
        }
    }
    pub fn body(mut self, body: &str) -> RequestBuilder {
        // todo: body should be bytes
        self.request.http_request_body_type =
            Some(HttpRequestBodyType::HttpRequestBody(body.to_owned()));
        self
    }
    pub fn text(mut self, text: &str) -> RequestBuilder {
        self.request.http_request_body_type =
            Some(HttpRequestBodyType::HttpRequestText(text.to_owned()));
        self
    }
    pub async fn send(self) -> Result<Response, Box<dyn Error>> {
        // Use Reqwest client to POST to Zyte API
        let mut response = self
            .client
            .client
            .post(&self.client.api_url)
            .basic_auth(&self.client.api_key, Some(""))
            .json(&self.request)
            .send()
            .await?
            .json::<Response>()
            .await?;

        // decode http_response_body from base 64
        let b = &general_purpose::STANDARD.decode(response.http_response_body)?;

        // Convert (unicode) byte vec (Vec<u8>) to String, ignoring errors
        let b = String::from_utf8_lossy(b);

        // Convert to real String and insert back into Response
        response.http_response_body = b.to_string(); // TODO: do this with serde

        Ok(response)
    }
}

#[derive(Clone)]
pub struct ZyteApi {
    client: reqwest::Client,
    api_key: String,
    api_url: String,
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
