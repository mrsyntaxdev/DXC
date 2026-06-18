# DXC — Download Anything.

A modern CLI for downloading videos, audio, and images from the web. Supports YouTube, TikTok, Instagram, Twitter/X, Pinterest, Reddit, Facebook, Vimeo, SoundCloud, Bilibili, and generic HTTP downloads.

## Install

```bash
cargo install --path crates/dxc-cli
```

Requires [yt-dlp](https://github.com/yt-dlp/yt-dlp) for site-specific providers and [ffmpeg](https://ffmpeg.org/) for media conversion.

## Usage

```bash
dxc download <url>          # Download video
dxc audio <url>             # Extract audio
dxc image <url>             # Download image
dxc info <url>              # Show media info
dxc convert <input> <fmt>   # Convert media
dxc history                 # View download history
dxc cache clear             # Clear cache
dxc config set <key> <val>  # Set config
dxc config get <key>        # Get config
```

## Supported Providers

| Provider   | Type    | Backend |
|------------|---------|---------|
| YouTube    | Video   | yt-dlp  |
| TikTok     | Video   | yt-dlp  |
| Instagram  | Image   | yt-dlp  |
| Twitter/X  | Image   | yt-dlp  |
| Pinterest  | Image   | yt-dlp  |
| Reddit     | Video   | API     |
| Facebook   | Video   | yt-dlp  |
| Vimeo      | Video   | yt-dlp  |
| SoundCloud | Audio   | yt-dlp  |
| Bilibili   | Video   | yt-dlp  |
| Generic    | Any     | HTTP    |

## Configuration

```bash
dxc config get download_path   # ~/Downloads/DXC/
dxc config set download_path ~/meu/personalizado
```

## Project Structure

```
crates/
├── dxc-cli/        # CLI interface (clap + colored + indicatif)
├── dxc-core/       # Types, config, errors
├── dxc-providers/  # 11 media source providers
├── dxc-db/         # SQLite history (rusqlite)
├── dxc-converter/  # FFmpeg wrapper
└── dxc-utils/      # Formatting helpers
```

## License

MIT
