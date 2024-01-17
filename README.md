# boaviztapi-sdk-rust

Rust client library for [Boaviztapi](https://github.com/Boavizta/boaviztapi).

## Versions supported

- SDK version 1.2.0 supports Boavizta API 1.2.0
- SDK version 1.1.0 supports Boavizta API 1.1.0
- SDK version 1.0.1 supports Boavizta API 1.0.1 (and historical 0.3.x series)
- SDK version 0.3.X supports Boavizta API of the 0.3.x series
- SDK version 0.2.x support Boavizta API of the 0.2.x series (Neither SDK nor API will not be updated after release of Boavizta API v0.3.x - around June 2023)

## Documentation

- SDK documentation (crate) [boavizta_api_sdk - Rust](https://docs.rs/boavizta_api_sdk/latest/boavizta_api_sdk/)
- Boavizta [API documentation](https://doc.api.boavizta.org/)

## How to generate  the sdk for the latest version of  BoaviztaAPI

SDK is generated from the published openAPI specification of Boaviztapi (<http://api.boavizta.org/openapi.json>).

We use openapi-generator-cli to generate the SDK. See [GitHub - OpenAPITools/openapi-generator-cli: A node package wrapper for https://github.com/OpenAPITools/openapi-generator](https://github.com/OpenAPITools/openapi-generator-cli) .

```sh
# Generate for public API
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i http://api.boavizta.org/openapi.json   -g rust  -o /local/ --package-name boavizta_api_sdk
# Local API (dev, using network host /!\)
docker run --network=host --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i http://localhost:5000/openapi.json  -g rust  -o /local/ --package-name boavizta_api_sdk
```

The generated code require some manual updates before being usable.

## How to generate SDK for a specific version of API

To generate the SDK for a *specific* version of BoaviztaAPI (e.g.  a dev branch or unreleased version), you have to:

1. retrieve the openapi.yml specification of the API
2. run the generator against this file
3. if needed publish the generated SDK wth an alpha version

### Update the Cargo.toml before publishing the SDK

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
## How to publish the SDK

This is a manual action and should be done only by the maintainer of the SDK.

```sh
cargo login
cargo publish --dry-run
cargo publish
```
