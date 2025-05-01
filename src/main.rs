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
