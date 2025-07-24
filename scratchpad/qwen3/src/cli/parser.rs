//! CLI argument parser using clap
//! Version: 0.1.0

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enable debug output
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    /// Audio input device
    #[arg(short, long)]
    pub audio_device: Option<String>,

    /// Preset directory path
    #[arg(long)]
    pub preset_dir: Option<String>,

    /// Start in fullscreen mode
    #[arg(long, default_value_t = false)]
    pub fullscreen: bool,

    /// Recording output directory
    #[arg(short, long)]
    pub record_dir: Option<String>,

    /// Enable text overlay
    #[arg(long, default_value_t = false)]
    pub overlay: bool,
}
