use async_trait::async_trait;
use dxc_core::{DxcError, MediaInfo, MediaType};

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &'static str;
    fn can_handle(&self, url: &str) -> bool;
    fn media_type(&self) -> MediaType;
    async fn fetch_info(&self, url: &str) -> Result<MediaInfo, DxcError>;
    async fn download(&self, url: &str, output_path: &str) -> Result<String, DxcError>;
}

pub fn resolve_provider(url: &str) -> Option<Box<dyn Provider>> {
    let providers: Vec<Box<dyn Provider>> = vec![
        Box::new(crate::youtube::YouTubeProvider),
        Box::new(crate::tiktok::TikTokProvider),
        Box::new(crate::instagram::InstagramProvider),
        Box::new(crate::twitter::TwitterProvider),
        Box::new(crate::pinterest::PinterestProvider),
        Box::new(crate::reddit::RedditProvider),
        Box::new(crate::facebook::FacebookProvider),
        Box::new(crate::vimeo::VimeoProvider),
        Box::new(crate::soundcloud::SoundCloudProvider),
        Box::new(crate::bilibili::BilibiliProvider),
        Box::new(crate::generic::GenericProvider),
    ];

    providers.into_iter().find(|p| p.can_handle(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_youtube() {
        let p = resolve_provider("https://youtube.com/watch?v=123").unwrap();
        assert_eq!(p.name(), "youtube");

        let p = resolve_provider("https://youtu.be/abc123").unwrap();
        assert_eq!(p.name(), "youtube");
    }

    #[test]
    fn test_resolve_tiktok() {
        let p = resolve_provider("https://www.tiktok.com/@user/video/123").unwrap();
        assert_eq!(p.name(), "tiktok");
    }

    #[test]
    fn test_resolve_instagram() {
        let p = resolve_provider("https://instagram.com/p/abc123").unwrap();
        assert_eq!(p.name(), "instagram");
    }

    #[test]
    fn test_resolve_twitter() {
        let p = resolve_provider("https://twitter.com/user/status/123").unwrap();
        assert_eq!(p.name(), "twitter");

        let p = resolve_provider("https://x.com/user/status/123").unwrap();
        assert_eq!(p.name(), "twitter");
    }

    #[test]
    fn test_resolve_pinterest() {
        let p = resolve_provider("https://pinterest.com/pin/123").unwrap();
        assert_eq!(p.name(), "pinterest");

        let p = resolve_provider("https://pin.it/abc").unwrap();
        assert_eq!(p.name(), "pinterest");
    }

    #[test]
    fn test_resolve_reddit() {
        let p = resolve_provider("https://www.reddit.com/r/rust/comments/123").unwrap();
        assert_eq!(p.name(), "reddit");
    }

    #[test]
    fn test_resolve_generic_fallback() {
        let p = resolve_provider("https://example.com/file.zip").unwrap();
        assert_eq!(p.name(), "generic");
    }

    #[test]
    fn test_resolve_facebook() {
        let p = resolve_provider("https://facebook.com/watch?v=123").unwrap();
        assert_eq!(p.name(), "facebook");

        let p = resolve_provider("https://fb.watch/abc").unwrap();
        assert_eq!(p.name(), "facebook");
    }

    #[test]
    fn test_resolve_vimeo() {
        let p = resolve_provider("https://vimeo.com/123456").unwrap();
        assert_eq!(p.name(), "vimeo");
    }

    #[test]
    fn test_resolve_soundcloud() {
        let p = resolve_provider("https://soundcloud.com/artist/track").unwrap();
        assert_eq!(p.name(), "soundcloud");
    }

    #[test]
    fn test_resolve_bilibili() {
        let p = resolve_provider("https://www.bilibili.com/video/BV1xx").unwrap();
        assert_eq!(p.name(), "bilibili");

        let p = resolve_provider("https://b23.tv/abc").unwrap();
        assert_eq!(p.name(), "bilibili");
    }

    #[test]
    fn test_provider_media_types() {
        assert_eq!(resolve_provider("https://youtube.com/").unwrap().media_type(), MediaType::Video);
        assert_eq!(resolve_provider("https://tiktok.com/").unwrap().media_type(), MediaType::Video);
        assert_eq!(resolve_provider("https://instagram.com/").unwrap().media_type(), MediaType::Image);
        assert_eq!(resolve_provider("https://twitter.com/").unwrap().media_type(), MediaType::Image);
        assert_eq!(resolve_provider("https://pinterest.com/").unwrap().media_type(), MediaType::Image);
        assert_eq!(resolve_provider("https://reddit.com/").unwrap().media_type(), MediaType::Video);
        assert_eq!(resolve_provider("https://facebook.com/").unwrap().media_type(), MediaType::Video);
        assert_eq!(resolve_provider("https://vimeo.com/").unwrap().media_type(), MediaType::Video);
        assert_eq!(resolve_provider("https://soundcloud.com/").unwrap().media_type(), MediaType::Audio);
        assert_eq!(resolve_provider("https://bilibili.com/").unwrap().media_type(), MediaType::Video);
    }
}
