use crate::Response;
use crate::ZyteApi;
use crate::{HttpRequestBodyType, Request};
use base64::{engine::general_purpose, Engine as _};
use http::Method;
use std::error::Error;

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
