#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize and build CKAN struct
    let ckan = ckanaction::CKAN::builder()
        .url("http://localhost:5000")
        .token("CKAN_API_TOKEN".to_string())
        .build();

    // Send request to /status_show and print output
    let status_show = ckan.status_show().await?;
    println!("{status_show:#?}");

    Ok(())
}
