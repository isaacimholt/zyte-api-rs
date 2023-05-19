//! # zyte-api-rs
//!
//! This is an unofficial rust package for the Zyte API.
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(with = "http_serde::uri")]
    url: http::Uri,
    http_response_body: Option<bool>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(with = "http_serde::uri")]
    pub url: http::Uri,
    pub http_response_body: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

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
        let request = Request {
            url: http::Uri::from_str(url)?,
            http_response_body: Some(true),
            ..Default::default()
        };

        // Use Reqwest client to POST to Zyte API and get page source
        let mut response = self
            .client
            .post(&self.api_url)
            .basic_auth(&self.api_key, Some(""))
            .json(&request)
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
