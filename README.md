<!-- voltaria-readme-assembled -->
# Voltaria Rust Library

## Quickstart

Install the SDK. The client is async, so you'll also want an async runtime such as [Tokio](https://tokio.rs):

```sh
cargo add voltaria-sdk
cargo add tokio --features full
```

Construct a client with your API key. The environment is resolved automatically from the key prefix:

```rust
use voltaria_sdk::api::types::ListLoansQueryRequest;
use voltaria_sdk::voltaria::Voltaria;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Production (live_ prefix).
    let client = Voltaria::new("live_...")?;

    // Sandbox (sandbox_ prefix).
    // let client = Voltaria::new("sandbox_...")?;

    let loans = client
        .loans
        .list_loans(&ListLoansQueryRequest::default(), None)
        .await?;

    println!("{loans:?}");

    Ok(())
}
```

The environment is selected from the API key prefix:

| Key prefix | Environment | Base URL                          |
| ---------- | ----------- | --------------------------------- |
| `live_`    | Production  | `https://api.voltaria.io`         |
| `sandbox_` | Sandbox     | `https://api.sandbox.voltaria.io` |

Use `Voltaria::builder(...)` to override the prefix-derived routing (custom environment or base URL).

See [reference.md](./reference.md) for the full API reference.

---

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-Built%20with%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=Voltaria%2FRust)
[![crates.io shield](https://img.shields.io/crates/v/voltaria_api)](https://crates.io/crates/voltaria_api)

The Voltaria Rust library provides convenient access to the Voltaria APIs from Rust.

## Table of Contents

- [Installation](#installation)
- [Reference](#reference)
- [Usage](#usage)
- [Environments](#environments)
- [Errors](#errors)
- [Request Types](#request-types)
- [Advanced](#advanced)
  - [Retries](#retries)
  - [Timeouts](#timeouts)
  - [Additional Headers](#additional-headers)
  - [Additional Query String Parameters](#additional-query-string-parameters)
- [Contributing](#contributing)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
voltaria_api = "0.1.0"
```

Or install via cargo:

```sh
cargo add voltaria_api
```

## Reference

A full reference for this library is available [here](./reference.md).

## Usage

Instantiate and use the client with the following:

```rust
use voltaria_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .clients
        .create_client(
            &ClientCreatePayload {
                name: "ACME Corp".to_string(),
                jurisdiction: JurisdictionEnum::Eu,
                correlation_id: None,
                r#type: None,
                company_number: None,
                data: None,
            },
            None,
        )
        .await;
}
```

## Environments

This SDK allows you to configure different environments for API requests.

```rust
use voltaria_api::prelude::{*};

let config = ClientConfig {
    base_url: Environment::Sandbox.url().to_string(),
    ..Default::default()
};
let client = Client::new(config).expect("Failed to build client");
```

## Errors

When the API returns a non-success status code (4xx or 5xx response), an error will be returned.

```rust
match client.clients.create_client(None)?.await {
    Ok(response) => {
        println!("Success: {:?}", response);
    },
    Err(ApiError::HTTP { status, message }) => {
        println!("API Error {}: {:?}", status, message);
    },
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

## Request Types

The SDK exports all request types as Rust structs. Simply import them from the crate to access them:

```rust
use voltaria_api::prelude::{*};

let request = ClientCreatePayload {
    ...
};
```

## Advanced

### Retries

The SDK is instrumented with automatic retries with exponential backoff. A request will be retried as long
as the request is deemed retryable and the number of retry attempts has not grown larger than the configured
retry limit (default: 2).

A request is deemed retryable when any of the following HTTP status codes is returned:

- [408](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/408) (Timeout)
- [429](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/429) (Too Many Requests)
- [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) (Internal Server Error)

The `retryStatusCodes` configuration controls which [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) status codes are retried:

- `legacy` (default): Retries `408`, `429`, and all `>= 500`
- `recommended`: Retries `408`, `429`, `502`, `503`, `504` only (excludes `500 Internal Server Error` to avoid retrying non-idempotent failures)

Use the `max_retries` method to configure this behavior.

```rust
let response = client.clients.create_client(
    Some(RequestOptions::new().max_retries(3))
)?.await;
```

### Timeouts

The SDK defaults to a 30 second timeout. Use the `timeout` method to configure this behavior.

```rust
let response = client.clients.create_client(
    Some(RequestOptions::new().timeout_seconds(30))
)?.await;
```

### Additional Headers

You can add custom headers to requests using `RequestOptions`.

```rust
let response = client.clients.create_client(
    Some(
        RequestOptions::new()
            .additional_header("X-Custom-Header", "custom-value")
            .additional_header("X-Another-Header", "another-value")
    )
)?
.await;
```

### Additional Query String Parameters

You can add custom query parameters to requests using `RequestOptions`.

```rust
let response = client.clients.create_client(
    Some(
        RequestOptions::new()
            .additional_query_param("filter", "active")
            .additional_query_param("sort", "desc")
    )
)?
.await;
```

## Contributing

While we value open-source contributions to this SDK, this library is generated programmatically.
Additions made directly to this library would have to be moved over to our generation code,
otherwise they would be overwritten upon the next generated release. Feel free to open a PR as
a proof of concept, but know that we will not be able to merge it as-is. We suggest opening
an issue first to discuss with us!

On the other hand, contributions to the README are always very welcome!
