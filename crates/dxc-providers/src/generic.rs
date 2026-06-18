use async_trait::async_trait;
use dxc_core::{DxcError, MediaInfo, MediaType};
use crate::provider::Provider;
use std::path::Path;

pub struct GenericProvider;

#[async_trait]
impl Provider for GenericProvider {
    fn name(&self) -> &'static str {
        "generic"
    }

    fn can_handle(&self, _url: &str) -> bool {
        true
    }

    fn media_type(&self) -> MediaType {
        MediaType::Video
    }

    async fn fetch_info(&self, url: &str) -> Result<MediaInfo, DxcError> {
        let client = reqwest::Client::builder()
            .user_agent("DXC/0.1")
            .build()
            .map_err(|e| DxcError::Network(e.to_string()))?;

        let resp = client
            .head(url)
            .send()
            .await
            .map_err(|e| DxcError::Network(e.to_string()))?;

        let size = resp
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .map(|b| dxc_utils::format_bytes(b))
            .unwrap_or_else(|| "Unknown".to_string());

        let filename = url
            .split('/')
            .last()
            .unwrap_or("file")
            .split('?')
            .next()
            .unwrap_or("file")
            .to_string();

        Ok(MediaInfo {
            title: filename,
            duration: "Unknown".to_string(),
            size,
            provider: "generic".to_string(),
        })
    }

    async fn download(&self, url: &str, output_path: &str) -> Result<String, DxcError> {
        let client = reqwest::Client::builder()
            .user_agent("DXC/0.1")
            .build()
            .map_err(|e| DxcError::Network(e.to_string()))?;

        let resp = client
            .get(url)
            .send()
            .await
            .map_err(|e| DxcError::Network(e.to_string()))?;

        let total = resp
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        let path = Path::new(output_path);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| DxcError::DownloadFailed(e.to_string()))?;
        }

        let mut file = tokio::fs::File::create(path)
            .await
            .map_err(|e| DxcError::DownloadFailed(e.to_string()))?;

        let mut downloaded: u64 = 0;
        let mut stream = resp.bytes_stream();

        use tokio::io::AsyncWriteExt;
        use tokio_stream::StreamExt;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| DxcError::Network(e.to_string()))?;
            file.write_all(&chunk)
                .await
                .map_err(|e| DxcError::DownloadFailed(e.to_string()))?;
            downloaded += chunk.len() as u64;

            if total > 0 {
                let pct = (downloaded as f64 / total as f64) * 100.0;
                print!(
                    "\r  Progress: {:.1}% ({}/{})",
                    pct,
                    dxc_utils::format_bytes(downloaded),
                    dxc_utils::format_bytes(total)
                );
            } else {
                print!("\r  Downloaded: {}", dxc_utils::format_bytes(downloaded));
            }
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
        println!();

        Ok(output_path.to_string())
    }
}
