use async_trait::async_trait;
use dxc_core::{DxcError, MediaInfo, MediaType};
use crate::provider::Provider;

pub struct YouTubeProvider;

#[async_trait]
impl Provider for YouTubeProvider {
    fn name(&self) -> &'static str {
        "youtube"
    }

    fn can_handle(&self, url: &str) -> bool {
        url.contains("youtube.com") || url.contains("youtu.be")
    }

    fn media_type(&self) -> MediaType {
        MediaType::Video
    }

    async fn fetch_info(&self, url: &str) -> Result<MediaInfo, DxcError> {
        let mut info = crate::ytdlp::fetch_info(url).await?;
        info.provider = "youtube".to_string();
        Ok(info)
    }

    async fn download(&self, url: &str, output_path: &str) -> Result<String, DxcError> {
        crate::ytdlp::download(url, output_path).await
    }
}
