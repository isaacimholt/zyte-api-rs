use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Response {
    #[serde(with = "http_serde::uri")]
    pub url: http::Uri,
    pub http_response_body: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}