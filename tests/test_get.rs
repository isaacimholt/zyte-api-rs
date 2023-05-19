use dotenv::dotenv;
use zyte_api_rs::ZyteApi;

#[tokio::test]
async fn test_get() {
    dotenv().ok();
    let zyte_api_key = std::env::var("ZYTE_API_KEY").unwrap();
    let zyte_api = ZyteApi::new(zyte_api_key.as_str());
    let response = zyte_api.get("https://www.google.com/").await.unwrap();
    assert!(response.status_code.is_success());
    assert_ne!(response.http_response_body, "".to_string());
}
