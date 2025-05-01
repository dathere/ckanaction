# ckanaction

Rust library crate to access CKAN Action API endpoints through Rust builders. Based on the CKAN Action API v3. Endpoints are expected to return with an output of type `serde_json::Value`.

## Examples

Run `/package_list` endpoint with a limit of 5 results per page and print the output:

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

    // Send request to /package_list and print output
    let result = ckan.package_list()
        .limit(5) // <-- This is an optional parameter you can remove
        .call()
        .await?;
    println!("{result:#?}");

    Ok(())
}
```

> The following examples won't include the boilerplate code.

Create a new package (dataset) with custom fields:

```rust
let custom_fields = serde_json::json!({
    "data_contact_email": "support@dathere.com",
    "update_frequency": "daily",
    "related_resources": [],
});
let result = ckan.package_create()
    .name("my-new-package".to_string())
    .custom_fields(custom_fields)
    .private(false)
    .call()
    .await?;
println!("{result:#?}");
```

Create a new resource with a new file from a file path:

```rust
let path_buf = current_dir()?.join("data.csv");
let result = ckan
    .resource_create()
    .package_id("3mz0qhbb-cdb0-ewst-x7c0-casnkwv0edub".to_string())
    .name("My new resource".to_string())
    .format("CSV".to_string())
    .upload(path_buf)
    .call()
    .await?;
println!("{result:#?}");
```

Some endpoints without any parameters may not need a builder such as `/status_show` so there is no `.call()` method after `.status_show()`:

```rust
let status_show = ckan.status_show().await?;
println!("{status_show:#?}");
```

## Notes

- Add the `CKAN_API_TOKEN` environment variable to a `.env` file where the program runs to include the token when making requests to the CKAN API.
- If you use a `maybe_fn()` then if you provide `None` it will be ignored and that parameter will not be added to the JSON body. This library assumes `None` would not be provided as a value (since the cases where it is a value is often the default value that the CKAN API already has set for that parameter).
