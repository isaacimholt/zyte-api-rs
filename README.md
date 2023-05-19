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

## Usage

```rust
let zyte_api: ZyteApi = ZyteApi::new("<MY-API-KEY-HERE>".to_string());
let response = zyte_api.get("https://www.google.com/").await;
```
