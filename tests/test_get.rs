use dotenv::dotenv;
use zyte_api_rs::ZyteApi;

#[tokio::test]
async fn test_http_get() {
    dotenv().ok();
    let zyte_api_key = std::env::var("ZYTE_API_KEY").unwrap();
    let zyte_api = ZyteApi::new(zyte_api_key.as_str());
    let response = zyte_api.get("https://www.google.com/").await.unwrap();
    assert!(response.status_code.is_success());
    assert_ne!(response.http_response_body, "".to_string());
}

#[tokio::test]
async fn test_http_post_text() {
    use assert_json_diff::assert_json_include;
    dotenv().ok();
    let zyte_api_key = std::env::var("ZYTE_API_KEY").unwrap();
    let zyte_api = ZyteApi::new(zyte_api_key.as_str());
    let response = zyte_api
        .post("https://httpbin.org/post")
        .unwrap()
        .text(r#"{"custname": "foobar"}"#)
        .send()
        .await
        .unwrap();
    assert!(response.status_code.is_success());

    let response_json: serde_json::Value =
        serde_json::from_str(&response.http_response_body).expect("JSON was not well-formatted");
    println!("{:?}", &response_json);
    assert_json_include!(
        actual:response_json,
        expected:serde_json::json!({"json": {"custname": "foobar"}}),
    )
}
