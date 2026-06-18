use std::process::Command;

#[derive(Debug)]
pub struct ConversionOptions {
    pub input: String,
    pub output_format: String,
}

pub fn convert(options: ConversionOptions) -> Result<String, String> {
    let input = &options.input;
    let fmt = &options.output_format.to_lowercase();
    let output = format!("{}.{fmt}", strip_extension(input));

    let is_audio = matches!(fmt.as_str(), "mp3" | "aac" | "ogg" | "wav" | "flac" | "opus" | "m4a");

    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-i", input, "-y"]);

    if is_audio {
        cmd.args(["-vn", "-acodec"]);
        match fmt.as_str() {
            "mp3" => { cmd.args(["libmp3lame", "-q:a", "2"]); }
            "aac" => { cmd.args(["aac", "-b:a", "192k"]); }
            "ogg" => { cmd.args(["libvorbis", "-q:a", "4"]); }
            "flac" => { cmd.args(["flac"]); }
            "opus" => { cmd.args(["libopus", "-b:a", "128k"]); }
            "wav" => { cmd.args(["pcm_s16le"]); }
            "m4a" => { cmd.args(["aac", "-b:a", "192k"]); }
            _ => { cmd.args(["copy"]); }
        }
    }

    cmd.arg(&output);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run ffmpeg: {e}"))?;

    if status.success() {
        Ok(output)
    } else {
        Err("Conversion failed".into())
    }
}

fn strip_extension(path: &str) -> String {
    match path.rfind('.') {
        Some(pos) => path[..pos].to_string(),
        None => path.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_extension() {
        assert_eq!(strip_extension("video.mp4"), "video");
        assert_eq!(strip_extension("path/to/file.mp3"), "path/to/file");
        assert_eq!(strip_extension("no_ext"), "no_ext");
    }

    #[test]
    fn test_strip_extension_double_dot() {
        assert_eq!(strip_extension("file.tar.gz"), "file.tar");
    }
}
