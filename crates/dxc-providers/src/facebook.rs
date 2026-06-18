use async_trait::async_trait;
use dxc_core::{DxcError, MediaInfo, MediaType};
use crate::provider::Provider;

pub struct FacebookProvider;

#[async_trait]
impl Provider for FacebookProvider {
    fn name(&self) -> &'static str {
        "facebook"
    }

    fn can_handle(&self, url: &str) -> bool {
        url.contains("facebook.com") || url.contains("fb.watch") || url.contains("fb.com")
    }

    fn media_type(&self) -> MediaType {
        MediaType::Video
    }

    async fn fetch_info(&self, url: &str) -> Result<MediaInfo, DxcError> {
        let mut info = crate::ytdlp::fetch_info(url).await?;
        info.provider = "facebook".to_string();
        Ok(info)
    }

    async fn download(&self, url: &str, output_path: &str) -> Result<String, DxcError> {
        crate::ytdlp::download(url, output_path).await
    }
}
