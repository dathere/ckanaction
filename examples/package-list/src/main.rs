use ckanaction::CKANError;

#[tokio::main]
async fn main() -> Result<(), CKANError> {
    // Initialize and build CKAN struct
    let ckan = ckanaction::CKAN::builder()
        .url("http://localhost:5000")
        .build();

    // Send request to /package_list and print formatted debug output
    let package_list = ckan.package_list().call().await?;
    println!("{package_list:#?}");

    Ok(())
}
