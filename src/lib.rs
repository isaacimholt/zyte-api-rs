//! # zyte-api-rs
//!
//! This is an unofficial rust package for the Zyte API.
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
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
    url: http::Uri,
    http_response_body: String,
    #[serde(with = "http_serde::status_code")]
    status_code: http::StatusCode,
}

pub struct ZyteApi {
    client: reqwest::blocking::Client,
    api_key: String,
    api_url: String,
}
impl ZyteApi {
    pub fn new(api_key: String) -> ZyteApi {
        ZyteApi {
            client: reqwest::blocking::Client::new(),
            api_key,
            api_url: "https://api.zyte.com/v1/extract".to_string(),
        }
    }
    pub fn get(&self, url: &str) -> Response {
        let request = Request {
            url: http::Uri::from_str(url).unwrap(),
            http_response_body: Some(true),
            ..Default::default()
        };

        // Use Reqwest client to POST to Zyte API and get page source
        let json = self
            .client
            .post(&self.api_url)
            .basic_auth(&self.api_key, Some(""))
            .json(&request)
            .send()
            .unwrap()
            .text()
            .unwrap();

        // load json into Response struct
        let mut response: Response = serde_json::from_str(&json).unwrap();

        // decode http_response_body from base 64
        let b = &general_purpose::STANDARD
            .decode(response.http_response_body)
            .unwrap();

        // Convert (unicode) byte vec (Vec<u8>) to String, ignoring errors
        let b = String::from_utf8_lossy(b);

        // Convert to real String and insert back into Response
        response.http_response_body = b.to_string();

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}