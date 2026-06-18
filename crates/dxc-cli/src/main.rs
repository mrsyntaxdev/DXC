use clap::{Parser, Subcommand};
use colored::*;
use dxc_core::{DxcConfig, DxcError, MediaType, ensure_dirs};
use dxc_providers::resolve_provider;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(name = "dxc", about = "DXC — Download Anything.", version, disable_version_flag = true)]
struct Cli {
    #[arg(short = 'v', long = "version", help = "Print version")]
    version: bool,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Download a video from a URL
    Download { url: String },
    /// Extract audio from a media URL
    Audio { url: String },
    /// Download an image from a URL
    Image { url: String },
    /// Show media information for a URL
    Info { url: String },
    /// Convert media files between formats
    Convert {
        input: String,
        output_format: String,
    },
    /// View download history
    History,
    /// Cache management
    Cache {
        #[command(subcommand)]
        action: CacheCommand,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigCommand,
    },
}

#[derive(Subcommand)]
enum CacheCommand {
    /// Clear the download cache
    Clear,
}

#[derive(Subcommand)]
enum ConfigCommand {
    /// Set a configuration value
    Set { key: String, value: String },
    /// Get a configuration value
    Get { key: String },
}

fn header(text: &str) {
    println!("\n{}", text.bold().cyan());
    println!("{}", "─".repeat(text.len()).cyan());
}

fn success(text: &str) {
    println!("  {} {}", "✔".green(), text.green());
}

fn error_msg(text: &str) {
    eprintln!("  {} {}", "✘".red(), text.red());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = ensure_dirs() {
        eprintln!("  {} {}", "⚠".yellow(), format!("config dirs: {e}").yellow());
    }

    let mut config = DxcConfig::load();
    cleanup_part_files(&config.download_path);
    let cli = Cli::parse();

    if cli.version {
        println!("DXC v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    match cli.command {
        Some(Command::Download { url }) => cmd_download(&url, MediaType::Video, &config).await?,
        Some(Command::Audio { url }) => cmd_download(&url, MediaType::Audio, &config).await?,
        Some(Command::Image { url }) => cmd_download(&url, MediaType::Image, &config).await?,
        Some(Command::Info { url }) => cmd_info(&url).await?,
        Some(Command::Convert { input, output_format }) => cmd_convert(&input, &output_format)?,
        Some(Command::History) => cmd_history(&config)?,
        Some(Command::Cache { action }) => match action {
            CacheCommand::Clear => cmd_cache_clear(&config)?,
        },
        Some(Command::Config { action }) => match action {
            ConfigCommand::Set { key, value } => cmd_config_set(&mut config, &key, &value)?,
            ConfigCommand::Get { key } => cmd_config_get(&config, &key)?,
        },
        None => {
            // No subcommand and no --version; clap already shows help
        }
    }

    Ok(())
}

async fn resolve_and_download(url: &str, output_path: &str) -> Result<(String, String, String), DxcError> {
    let provider = resolve_provider(url)
        .ok_or_else(|| DxcError::ProviderNotFound(url.to_string()))?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner} {msg}")
            .unwrap(),
    );
    spinner.set_message("Fetching info...");

    let info = provider.fetch_info(url).await?;

    spinner.finish_and_clear();

    println!("  {} {}", "⊙".cyan(), format!("Provider: {}", provider.name()).bold());
    println!("  {} {}", "⊙".cyan(), format!("Title:    {}", info.title).bold());
    println!("  {} {}", "⊙".cyan(), format!("Size:     {}", info.size).bold());
    if info.duration != "Unknown" {
        println!("  {} {}", "⊙".cyan(), format!("Duration: {}", info.duration).bold());
    }
    println!();

    let file_path = provider.download(url, output_path).await?;
    Ok((provider.name().to_string(), info.title.clone(), file_path))
}

async fn cmd_download(url: &str, media_type: MediaType, config: &DxcConfig) -> anyhow::Result<()> {
    header("DOWNLOAD");

    println!("  {} {}\n", "URL:".dimmed(), url);

    let output_dir = match media_type {
        MediaType::Video => format!("{}/videos", config.download_path),
        MediaType::Audio => format!("{}/audio", config.download_path),
        MediaType::Image => format!("{}/images", config.download_path),
    };

    let filename = dxc_utils::sanitize_filename(
        url.split('/').last().unwrap_or("download").split('?').next().unwrap_or("download")
    );
    let output_path = format!("{output_dir}/{filename}");

    let db = dxc_db::Database::open(&config.db_path).ok();

    match resolve_and_download(url, &output_path).await {
        Ok((provider, title, file_path)) => {
            if let Some(ref db) = db {
                let _ = db.insert_history(url, Some(&title), &format!("{:?}", media_type), &provider, Some(&file_path), true);
            }
            println!();
            success(&format!("Saved → {}", file_path.bold()));
        }
        Err(e) => {
            if let Some(ref db) = db {
                let _ = db.insert_history(url, None, &format!("{:?}", media_type), "unknown", None, false);
            }
            println!();
            error_msg(&e.to_string());
        }
    }

    println!();
    Ok(())
}

async fn cmd_info(url: &str) -> anyhow::Result<()> {
    let provider = match resolve_provider(url) {
        Some(p) => p,
        None => {
            error_msg(&format!("No provider found for URL: {url}"));
            return Ok(());
        }
    };

    header("INFO");

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner} {msg}")
            .unwrap(),
    );
    spinner.set_message("Fetching info...");

    match provider.fetch_info(url).await {
        Ok(info) => {
            spinner.finish_and_clear();
            println!("  {} {}\n", "URL:".dimmed(), url);
            println!("  {}  {}", "◉".cyan(), format!("Provider: {}", info.provider).bold());
            println!("  {}  {}", "◉".cyan(), format!("Title:    {}", info.title).bold());
            println!("  {}  {}", "◉".cyan(), format!("Duration: {}", info.duration).bold());
            println!("  {}  {}", "◉".cyan(), format!("Size:     {}", info.size).bold());
        }
        Err(e) => {
            spinner.finish_and_clear();
            error_msg(&e.to_string());
        }
    }

    println!();
    Ok(())
}

fn cmd_convert(input: &str, output_format: &str) -> anyhow::Result<()> {
    header("CONVERT");

    println!("  {}  {}", "Input:".dimmed(), input);
    println!("  {}  {}\n", "Format:".dimmed(), output_format);

    let options = dxc_converter::ConversionOptions {
        input: input.to_string(),
        output_format: output_format.to_string(),
    };

    match dxc_converter::convert(options) {
        Ok(output) => success(&format!("Created → {}", output.bold())),
        Err(e) => error_msg(&e),
    }

    println!();
    Ok(())
}

fn cmd_history(config: &DxcConfig) -> anyhow::Result<()> {
    header("HISTORY");

    let db = match dxc_db::Database::open(&config.db_path) {
        Ok(db) => db,
        Err(e) => {
            error_msg(&format!("Database: {e}"));
            return Ok(());
        }
    };

    let entries = db.get_history(20)?;

    if entries.is_empty() {
        println!("  No history yet.\n");
        return Ok(());
    }

    println!("  {}  {}  {}", format!("{:<20}", "DATE").bold().dimmed(), format!("{:<4}", "STATUS").bold().dimmed(), format!("{:<40}", "TITLE / URL").bold().dimmed());
    println!("  {}", "─".repeat(70).dimmed());

    for entry in &entries {
        let status = if entry.success {
            "OK".green()
        } else {
            "FAIL".red()
        };

        let date = &entry.created_at[..19];
        let display = entry.title.as_deref().unwrap_or(&entry.url);
        let truncated = if display.len() > 40 {
            format!("{}…", &display[..39])
        } else {
            display.to_string()
        };

        println!("  {}  {}  {}", date.dimmed(), status, truncated);
    }

    println!();
    Ok(())
}

fn cmd_cache_clear(config: &DxcConfig) -> anyhow::Result<()> {
    header("CACHE");

    match std::fs::remove_dir_all(&config.cache_path) {
        Ok(_) => {
            std::fs::create_dir_all(&config.cache_path).ok();
            success("Cache cleared.");
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("  Cache is already empty.");
        }
        Err(e) => {
            error_msg(&format!("Failed to clear cache: {e}"));
        }
    }

    println!();
    Ok(())
}

fn cmd_config_set(config: &mut DxcConfig, key: &str, value: &str) -> anyhow::Result<()> {
    match config.set(key, value) {
        Ok(_) => success(&format!("Set {} = {}", key.bold(), value.bold())),
        Err(e) => error_msg(&e),
    }
    Ok(())
}

fn cmd_config_get(config: &DxcConfig, key: &str) -> anyhow::Result<()> {
    match config.get(key) {
        Some(val) => println!("{}", val.bold()),
        None => error_msg(&format!("Unknown key: {key}")),
    }
    Ok(())
}

fn cleanup_part_files(download_path: &str) {
    let dirs = [
        format!("{download_path}/videos"),
        format!("{download_path}/audio"),
        format!("{download_path}/images"),
    ];
    for dir in &dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "part") {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
    }
}
