# zyte-api-rs

Allows access to the Zyte API proxy service.

This is an unofficial, unstable, unfinished crate. However normal usage with `HTTP GET` should work fine.

## Prerequisites

- You must have an account & API token from Zyte for their proxy API service: <https://www.zyte.com/zyte-api/>
- Currently this crate only supports `async`/`await` usage so you must use an async executor e.g. <https://github.com/tokio-rs/tokio>.

## Installation

```bash
cargo add zyte-api-rs
```

## Example

```rust
use zyte_api_rs::ZyteApi;

#[tokio::main]
async fn get_google() {
    let zyte_api = ZyteApi::new("<MY_ZYTE_API_KEY>");

    // simple GET
    let response = zyte_api.get("https://www.google.com/").await.unwrap();

    // status_code is from http::Method
    if response.status_code.is_success() {
        println!("{}", response.http_response_body);
    }

    let response = zyte_api
        .post("https://httpbin.org/post")
        .unwrap()
        .text(r#"{"custname": "foobar"}"#)
        .send()
        .await
        .unwrap();

    if response.status_code.is_success() {
        println!("{}", response.http_response_body);
    }

}

```

## Features

### Requests

- [ ] `HTTP`
  - [x] `GET`
  - [ ] `POST`
    - [ ] `httpResponseBody`
    - [ ] `httpResponseText`
- [ ] `Browser`
- [ ] `Headers`

### Responses

- [ ] Normal Responses
- [ ] Error Responses

## Design Goals

- Have `zyte-api-rs`'s `Response` object mirror (as much as possible) the structure of the `Response` schema from the official api: <https://docs.zyte.com/zyte-api/usage/reference.html>
- Have an easy-to-use "builder" interface similar to <https://github.com/seanmonstar/reqwest>
- The json structure of requests to the zyte api are a bit complicated, with mutually-exclusive keys for different features -- this project aims to "make invalid states unrepresentable"
- Learn Idiomatic Rust -- please file issues for code that is poorly written: <https://github.com/isaacimholt/zyte-api-rs/issues>.

## Notes

- The status code is actually an instance of [`http::StatusCode`](https://docs.rs/http/latest/http/status/struct.StatusCode.html) which allows more useful semantics such as `status_code.is_success()`.
