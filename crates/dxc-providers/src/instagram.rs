use async_trait::async_trait;
use dxc_core::{DxcError, MediaInfo, MediaType};
use crate::provider::Provider;

pub struct InstagramProvider;

#[async_trait]
impl Provider for InstagramProvider {
    fn name(&self) -> &'static str {
        "instagram"
    }

    fn can_handle(&self, url: &str) -> bool {
        url.contains("instagram.com")
    }

    fn media_type(&self) -> MediaType {
        MediaType::Image
    }

    async fn fetch_info(&self, url: &str) -> Result<MediaInfo, DxcError> {
        let mut info = crate::ytdlp::fetch_info(url).await?;
        info.provider = "instagram".to_string();
        Ok(info)
    }

    async fn download(&self, url: &str, output_path: &str) -> Result<String, DxcError> {
        crate::ytdlp::download_with_args(
            url, output_path,
            &["--extractor-args", "instagram:app_id=124024574287414"],
        ).await
    }
}
