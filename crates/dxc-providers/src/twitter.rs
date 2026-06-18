use async_trait::async_trait;
use dxc_core::{DxcError, MediaInfo, MediaType};
use crate::provider::Provider;

pub struct TwitterProvider;

#[async_trait]
impl Provider for TwitterProvider {
    fn name(&self) -> &'static str {
        "twitter"
    }

    fn can_handle(&self, url: &str) -> bool {
        url.contains("twitter.com") || url.contains("x.com")
    }

    fn media_type(&self) -> MediaType {
        MediaType::Image
    }

    async fn fetch_info(&self, url: &str) -> Result<MediaInfo, DxcError> {
        let mut info = crate::ytdlp::fetch_info(url).await?;
        info.provider = "twitter".to_string();
        Ok(info)
    }

    async fn download(&self, url: &str, output_path: &str) -> Result<String, DxcError> {
        crate::ytdlp::download(url, output_path).await
    }
}
