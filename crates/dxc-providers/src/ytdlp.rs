use dxc_core::DxcError;
use serde::Deserialize;
use std::path::Path;
use std::process::Stdio;
use dxc_core::MediaInfo;

#[derive(Deserialize)]
pub struct YtDlpInfo {
    pub title: Option<String>,
    pub duration: Option<f64>,
    #[serde(rename = "filesize_approx")]
    pub filesize: Option<u64>,
}

pub async fn fetch_info(url: &str) -> Result<MediaInfo, DxcError> {
    let output = tokio::process::Command::new("yt-dlp")
        .args(["--no-update", "--dump-json", "--no-download", url])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
        .map_err(|e| DxcError::Other(format!("yt-dlp not found: {e}")))?;

    if !output.status.success() {
        return Err(DxcError::Other("yt-dlp failed to fetch info".into()));
    }

    let info: YtDlpInfo = serde_json::from_slice(&output.stdout)
        .map_err(|e| DxcError::Other(format!("Failed to parse yt-dlp output: {e}")))?;

    let duration = info.duration
        .map(|s| dxc_utils::format_duration(s as u64))
        .unwrap_or_else(|| "Unknown".to_string());

    let size = info.filesize
        .map(|b| dxc_utils::format_bytes(b))
        .unwrap_or_else(|| "Unknown".to_string());

    Ok(MediaInfo {
        title: info.title.unwrap_or_else(|| "Unknown".to_string()),
        duration,
        size,
        provider: "unknown".to_string(),
    })
}

pub async fn download(url: &str, output_path: &str) -> Result<String, DxcError> {
    let dir = Path::new(output_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());

    let output = tokio::process::Command::new("yt-dlp")
        .args([
            "--no-update",
            "--merge-output-format", "mp4",
            "-o", &format!("{}/%(title)s.%(ext)s", dir),
            "--print", "after_move:filepath",
            "--no-playlist",
            url,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .await
        .map_err(|e| DxcError::Other(format!("yt-dlp not found: {e}")))?;

    if output.status.success() {
        let actual_path = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        if actual_path.is_empty() {
            Err(DxcError::DownloadFailed("yt-dlp returned no output path".into()))
        } else {
            Ok(actual_path)
        }
    } else {
        Err(DxcError::DownloadFailed("yt-dlp download failed".into()))
    }
}
