use ckanaction::CKAN;

const CKAN_API_TOKEN: &str = "";
const CKAN_URL: &str = "";

pub async fn get_ckan_builder() -> CKAN {
    CKAN::builder()
        .token(CKAN_API_TOKEN.to_string())
        .url(CKAN_URL)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn status_show() -> Result<(), Box<dyn std::error::Error>> {
        let ckan = get_ckan_builder().await;
        let response = ckan.status_show().await?;
        assert!(response.is_object());
        let success = response.as_object().unwrap().get("success").unwrap().as_bool().unwrap();
        assert!(success);
        Ok(())
    }
}
