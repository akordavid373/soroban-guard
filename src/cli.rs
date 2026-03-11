use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "soroban-security-guard",
    about = "Automated invariant scanner for Soroban smart contracts",
    version = env!("CARGO_PKG_VERSION"),
    author = "Security Guard Team"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to configuration file
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Output format (console, json, html, sarif)
    #[arg(short, long, global = true, default_value = "console")]
    pub output: String,

    /// Minimum severity level (low, medium, high, critical)
    #[arg(short = 's', long, global = true, default_value = "medium")]
    pub severity: String,

    /// Output file path (default: stdout)
    #[arg(short, long, global = true)]
    pub output_file: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan a contract or directory for security issues
    Scan {
        /// Path to contract file or directory to scan
        path: PathBuf,

        /// Exclude files matching patterns
        #[arg(long, value_delimiter = ',')]
        exclude: Vec<String>,

        /// Include only files matching patterns
        #[arg(long, value_delimiter = ',')]
        include: Vec<String>,

        /// Maximum directory depth to scan
        #[arg(long)]
        max_depth: Option<usize>,
    },

    /// List available security rules
    ListRules {
        /// Filter rules by severity
        #[arg(long)]
        severity: Option<String>,

        /// Show disabled rules
        #[arg(long)]
        show_disabled: bool,
    },

    /// Generate configuration file
    InitConfig {
        /// Output path for config file
        #[arg(short, long, default_value = "soroban-security-guard.toml")]
        output: PathBuf,

        /// Use strict configuration
        #[arg(long)]
        strict: bool,
    },

    /// Validate configuration file
    ValidateConfig {
        /// Path to configuration file
        config: PathBuf,
    },

    /// Show version and build information
    Version {
        /// Show detailed version information
        #[arg(long)]
        detailed: bool,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
