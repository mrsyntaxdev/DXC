use async_trait::async_trait;
use dxc_core::{DxcError, MediaInfo, MediaType};
use crate::provider::Provider;
use serde::Deserialize;

pub struct RedditProvider;

#[derive(Deserialize)]
struct RedditResponse {
    data: RedditData,
}

#[derive(Deserialize)]
struct RedditData {
    children: Vec<RedditChild>,
}

#[derive(Deserialize)]
struct RedditChild {
    data: RedditPost,
}

#[derive(Deserialize)]
struct RedditPost {
    title: Option<String>,
    url_overridden_by_dest: Option<String>,
    domain: Option<String>,
    secure_media: Option<RedditMedia>,
}

#[derive(Deserialize)]
struct RedditMedia {
    reddit_video: Option<RedditVideo>,
}

#[derive(Deserialize)]
struct RedditVideo {
    fallback_url: Option<String>,
    duration: Option<f64>,
}

#[async_trait]
impl Provider for RedditProvider {
    fn name(&self) -> &'static str {
        "reddit"
    }

    fn can_handle(&self, url: &str) -> bool {
        url.contains("reddit.com")
    }

    fn media_type(&self) -> MediaType {
        MediaType::Video
    }

    async fn fetch_info(&self, url: &str) -> Result<MediaInfo, DxcError> {
        let json_url = if url.ends_with('/') {
            format!("{}reddit.json", url.strip_suffix('/').unwrap_or(url))
        } else if url.contains("/s/") {
            return Err(DxcError::Other("Reddit share links not yet supported".into()));
        } else {
            format!("{url}/.json")
        };

        let client = reqwest::Client::new()
            .get(&json_url)
            .header("User-Agent", "DXC/0.1");

        let resp = client
            .send()
            .await
            .map_err(|e| DxcError::Network(e.to_string()))?;

        let responses: Vec<RedditResponse> = resp
            .json()
            .await
            .map_err(|e| DxcError::Other(format!("Failed to parse Reddit API: {e}")))?;

        let post = responses
            .first()
            .and_then(|r| r.data.children.first())
            .map(|c| &c.data)
            .ok_or_else(|| DxcError::Other("No post data found".into()))?;

        let title = post.title.clone().unwrap_or_else(|| "Unknown".to_string());

        let duration = post.secure_media
            .as_ref()
            .and_then(|m| m.reddit_video.as_ref())
            .and_then(|v| v.duration)
            .map(|s| dxc_utils::format_duration(s as u64))
            .unwrap_or_else(|| "Unknown".to_string());

        Ok(MediaInfo {
            title,
            duration,
            size: "Unknown".to_string(),
            provider: "reddit".to_string(),
        })
    }

    async fn download(&self, url: &str, output_path: &str) -> Result<String, DxcError> {
        let json_url = if url.ends_with('/') {
            format!("{}reddit.json", url.strip_suffix('/').unwrap_or(url))
        } else {
            format!("{url}/.json")
        };

        let client = reqwest::Client::new()
            .get(&json_url)
            .header("User-Agent", "DXC/0.1");

        let resp = client
            .send()
            .await
            .map_err(|e| DxcError::Network(e.to_string()))?;

        let responses: Vec<RedditResponse> = resp
            .json()
            .await
            .map_err(|e| DxcError::Other(format!("Failed to parse Reddit API: {e}")))?;

        let post = responses
            .first()
            .and_then(|r| r.data.children.first())
            .map(|c| &c.data)
            .ok_or_else(|| DxcError::Other("No post data found".into()))?;

        if let Some(media) = &post.secure_media {
            if let Some(video) = &media.reddit_video {
                if let Some(ref video_url) = video.fallback_url {
                    let client = reqwest::Client::new();
                    let resp = client
                        .get(video_url)
                        .send()
                        .await
                        .map_err(|e| DxcError::Network(e.to_string()))?;

                    let total = resp
                        .headers()
                        .get(reqwest::header::CONTENT_LENGTH)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok())
                        .unwrap_or(0);

                    let path = std::path::Path::new(output_path);
                    if let Some(parent) = path.parent() {
                        tokio::fs::create_dir_all(parent)
                            .await
                            .map_err(|e| DxcError::DownloadFailed(e.to_string()))?;
                    }

                    let mut file = tokio::fs::File::create(path)
                        .await
                        .map_err(|e| DxcError::DownloadFailed(e.to_string()))?;

                    let mut downloaded: u64 = 0;
                    use tokio::io::AsyncWriteExt;
                    use tokio_stream::StreamExt;

                    let mut stream = resp.bytes_stream();
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
                        }
                        use std::io::Write;
                        std::io::stdout().flush().ok();
                    }
                    println!();

                    if let Some(ext) = video_url.rsplit('.').next() {
                        let final_path = format!("{output_path}.{ext}");
                        tokio::fs::rename(output_path, &final_path)
                            .await
                            .map_err(|e| DxcError::DownloadFailed(e.to_string()))?;
                        return Ok(final_path);
                    }
                    return Ok(output_path.to_string());
                }
            }
        }

        if let Some(ref url_str) = post.url_overridden_by_dest {
            if post.domain.as_deref() != Some("reddit.com") {
                let provider = crate::generic::GenericProvider;
                return provider.download(url_str, output_path).await;
            }
        }

        Err(DxcError::Other("No downloadable media found in Reddit post".into()))
    }
}
