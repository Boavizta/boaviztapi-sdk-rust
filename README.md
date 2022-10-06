# boaviztapi-sdk-rust

Rust client library for [Boaviztapi](https://github.com/Boavizta/boaviztapi).

## Documentation

Boavizta [API documentation](https://doc.api.boavizta.org/)

## How to generate / update the API

API is generated from the published openAPI specification of Boaviztapi (<http://api.boavizta.org/openapi.json>).

We use openapi-generator-cli to generate the SDK. See [GitHub - OpenAPITools/openapi-generator-cli: A node package wrapper for https://github.com/OpenAPITools/openapi-generator](https://github.com/OpenAPITools/openapi-generator-cli) .

```sh
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i http://api.boavizta.org/openapi.json   -g rust  -o /local/ --package-name boavizta_api_sdk
```

The generated code require some manual updates before being usable.

### Update the Cargo.toml

Metadata of Cargo.toml are overwritten by generation.

1. verify that version of package is consistent
1. update it with the following values

```toml
description = "A Rust client/sdk to access Boavizta API"
homepage = "https://boavizta.org"
repository = "https://github.com/Boavizta/boaviztapi-sdk-rust/"
readme = "README.md"
authors = ["boavizta.org", "olivier de Meringo"]
edition = "2018"
license-file = "LICENSE"
```

Ensure that the dependency on `reqwest`  does not use native SSL(the default feature), but uses  `rust-tls` . It makes cross compiling so much easier.

```toml
[dependencies.reqwest]
version = "^0.11"
default-features = false 
features = ["json", "multipart", "rustls-tls"]
```

#### Update generated code

Use `cargo clippy` or `cargo check` to check that the generated code compiles.

⚠ The following (useful when generating for API v0.1.2) does not seem necessary anymore with API v0.2.x series).

```sh
# Only use the following command on generated code for v0.1.x API
echo "⚠ Rename the field \`type`` into \`usage_type\` to comply with Rust naming conventions"
echo "⚠ This is really a hacky workaround that we should remove when the code generation is fixed" 
sed -i "s/pub type: Option<String>,/pub usage_type: Option<String>,/" src/models/usage_cloud.rs
sed -i "s/type: None,/usage_type: None,/" src/models/usage_cloud.rs
sed -i "s/pub type: Option<String>,/pub usage_type: Option<String>,/" src/models/usage_server.rs
sed -i "s/type: None,/usage_type: None,/" src/models/usage_server.rs
```
