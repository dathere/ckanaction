# ckanaction

Rust library crate to access CKAN Action API endpoints through Rust builders. Based on the CKAN Action API v3. Endpoints are expected to return with an output of type `serde_json::Value`.

## Example

```rust
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv()?;

    // Initialize and build CKAN struct
    let ckan = ckanaction::CKAN::builder()
        .url("http://localhost:5000")
        .token(dotenvy::var("CKAN_API_TOKEN")?)
        .build();

    // Send request to /status_show and print output
    let status_show = ckan.status_show().await?;
    println!("{status_show:#?}");

    Ok(())
}
```

## Notes

- Add the `CKAN_API_TOKEN` environment variable to a `.env` file where the program runs to include the token when making requests to the CKAN API.
- If you use a `maybe_fn()` then if you provide `None` it will be ignored and that parameter will not be added to the JSON body. This library assumes `None` would not be provided as a value (since the cases where it is a value is often the default value that the CKAN API already has set for that parameter).
