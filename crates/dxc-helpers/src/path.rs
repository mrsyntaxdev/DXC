pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
            c
        } else {
            '_'
        })
        .collect()
}

pub fn file_extension(path: &str) -> Option<&str> {
    path.rfind('.').map(|pos| &path[pos + 1..])
}
