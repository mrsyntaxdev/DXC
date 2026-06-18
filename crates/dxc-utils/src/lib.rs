pub mod path;
pub mod format;

pub use path::*;
pub use format::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_format_bytes() {
        use crate::format_bytes;
        assert_eq!(format_bytes(0), "0.00 B");
        assert_eq!(format_bytes(1023), "1023.00 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_format_duration() {
        use crate::format_duration;
        assert_eq!(format_duration(0), "0:00");
        assert_eq!(format_duration(59), "0:59");
        assert_eq!(format_duration(60), "1:00");
        assert_eq!(format_duration(3661), "1:01:01");
        assert_eq!(format_duration(86399), "23:59:59");
    }

    #[test]
    fn test_sanitize_filename() {
        use crate::sanitize_filename;
        assert_eq!(sanitize_filename("hello.txt"), "hello.txt");
        assert_eq!(sanitize_filename("a/b:c"), "a_b_c");
        assert_eq!(sanitize_filename("normal-file_1.mp4"), "normal-file_1.mp4");
    }

    #[test]
    fn test_file_extension() {
        use crate::file_extension;
        assert_eq!(file_extension("file.mp4"), Some("mp4"));
        assert_eq!(file_extension("file.tar.gz"), Some("gz"));
        assert_eq!(file_extension("no_ext"), None);
    }
}
