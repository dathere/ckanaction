use ckanaction::CKANError;

#[tokio::main]
async fn main() -> Result<(), CKANError> {
    // Initialize and build CKAN struct
    let ckan = ckanaction::CKAN::builder()
        .url("http://localhost:5000")
        .build();

    // Send request to /status_show and print formatted debug output
    let status_show = ckan.status_show().await?;
    println!("{status_show:#?}");

    Ok(())
}
