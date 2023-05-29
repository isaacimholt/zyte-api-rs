use serde::{Serialize, Serializer};


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum HttpRequestBodyType {
    HttpRequestBody(String), // todo: body should be bytes
    HttpRequestText(String),
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(with = "http_serde::uri")]
    pub url: http::Uri,
    pub http_response_body: Option<bool>,
    #[serde(serialize_with = "method_opt")]
    pub http_request_method: Option<http::Method>,
    #[serde(flatten)]
    pub http_request_body_type: Option<HttpRequestBodyType>,
}

/// https://stackoverflow.com/a/76143471/196870
fn method_opt<S: Serializer>(method_opt: &Option<http::Method>, ser: S) -> Result<S::Ok, S::Error> {
    match method_opt {
        Some(method) => http_serde::method::serialize(method, ser),
        // This won't be encountered when using skip_serializing_none
        None => ser.serialize_none(),
    }
}