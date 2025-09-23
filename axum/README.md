# Limiting Factor for axum

This crate helps to build REST API with axum, with less boilerplate code.

## Implemented features
### Extractors

The extractor for request body is a port of the Rocket 0.4 guard added in Limiting Factor 0.8.0.

If you need to read the body of the HTTP request "as is", the AxumRequestBody
extractor allows you to read it as a string:

    async fn deploy(
        Path(site_name): Path<String>,
        State(config): State<AlkaneConfig>,
        body: AxumRequestBody,
    ) -> ApiResult<Json<RecipeStatus>> {
        let context = body.into_optional_string(); // Option<String>
        // ...
    }

## Development

Current focus is to port features used by REST API from Rocket 0.4 to axum 0.8.4+.

New features:
  - may be added to the axum crate
  - should be added to the core crate for the abstract part
  - are not expected to be implemented to the rocket-legacy crate
