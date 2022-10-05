# boaviztapi-sdk-rust

Rust client library for [Boaviztapi](https://github.com/Boavizta/boaviztapi).

## Documentation

Boavizta [API documentation](https://doc.api.boavizta.org/)

## How to generate / update the API

API is generated from the published openAPI specification of Boaviztapi (<http://api.boavizta.org/openapi.json>).

We use openapi-generator-cli to generate the SDK. See [GitHub - OpenAPITools/openapi-generator-cli: A node package wrapper for https://github.com/OpenAPITools/openapi-generator](https://github.com/OpenAPITools/openapi-generator-cli) .

```sh
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i http://api.boavizta.org/openapi.json   -g rust  -o /local/boavizta-api-sdk --package-name boavizta_api_sdk
```

Note: the generated code require some manual updates before being usable.

```sh
echo "⚠ Rename the field \`type`` into \`usage_type\` to comply with Rust naming conventions"
echo "⚠ This is really a hacky workaround that we should remove when the code generation is fixed" 
sed -i "s/pub type: Option<String>,/pub usage_type: Option<String>,/" src/models/usage_cloud.rs
sed -i "s/type: None,/usage_type: None,/" src/models/usage_cloud.rs
sed -i "s/pub type: Option<String>,/pub usage_type: Option<String>,/" src/models/usage_server.rs
sed -i "s/type: None,/usage_type: None,/" src/models/usage_server.rs
```
