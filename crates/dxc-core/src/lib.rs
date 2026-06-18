pub mod media;
pub mod error;
pub mod config;

pub use media::*;
pub use error::*;
pub use config::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_media_type_serialization() {
        use crate::MediaType;
        assert_eq!(format!("{:?}", MediaType::Video), "Video");
        assert_eq!(format!("{:?}", MediaType::Audio), "Audio");
        assert_eq!(format!("{:?}", MediaType::Image), "Image");
    }

    #[test]
    fn test_dirs_defaults() {
        use crate::dirs;
        let config_dir = dirs::config_dir();
        assert!(config_dir.contains(".config/dxc"));
        let cache = dirs::cache();
        assert!(cache.contains(".cache/dxc"));
        let downloads = dirs::downloads();
        assert!(downloads.contains("Downloads/DXC"));
    }

    #[test]
    fn test_config_default() {
        use crate::DxcConfig;
        let config = DxcConfig::default();
        assert_eq!(config.max_concurrent_downloads, 3);
        assert!(config.download_path.contains("Downloads/DXC"));
    }
}
