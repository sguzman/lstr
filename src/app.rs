//! Defines the command-line interface for the lstr application.

use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;
use std::path::PathBuf;

/// A blazingly fast, minimalist directory tree viewer, written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    /// The subcommand to run. If no subcommand is specified, the classic tree view is displayed.
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// The arguments for the classic tree view. These are used when no subcommand is provided.
    #[command(flatten)]
    pub view: ViewArgs,
}

/// Defines the available subcommands for the application.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start the interactive TUI explorer.
    #[command(visible_alias = "i")]
    Interactive(InteractiveArgs),
}

/// Arguments for the classic `view` command.
#[derive(Parser, Debug, Default)]
pub struct ViewArgs {
    /// The path to the directory to display. Defaults to the current directory.
    #[arg(default_value = ".")]
    pub path: PathBuf,
    /// Specify when to use colorized output.
    #[arg(long, value_name = "WHEN", default_value_t = ColorChoice::Auto)]
    pub color: ColorChoice,
    /// Maximum depth to descend in the directory tree.
    #[arg(short = 'L', long)]
    pub level: Option<usize>,
    /// Display directories only.
    #[arg(short = 'd', long)]
    pub dirs_only: bool,
    /// Display the size of files.
    #[arg(short = 's', long)]
    pub size: bool,
    /// Show all files, including hidden ones.
    #[arg(short = 'a', long, help = "Show all files, including hidden ones")]
    pub all: bool,
    /// Respect .gitignore and other standard ignore files.
    #[arg(short = 'g', long)]
    pub gitignore: bool,
    /// Display file-specific icons (requires a Nerd Font).
    #[arg(long, help = "Display file-specific icons (requires a Nerd Font)")]
    pub icons: bool,
}

/// Arguments for the `interactive` command.
#[derive(Parser, Debug)]
pub struct InteractiveArgs {
    /// The path to the directory to explore. Defaults to the current directory.
    #[arg(default_value = ".")]
    pub path: PathBuf,
    /// Show all files, including hidden ones.
    #[arg(short = 'a', long)]
    pub all: bool,
    /// Respect .gitignore and other standard ignore files.
    #[arg(short = 'g', long)]
    pub gitignore: bool,
    /// Display file-specific icons (requires a Nerd Font).
    #[arg(long)]
    pub icons: bool,
    /// Display the size of files.
    #[arg(short = 's', long)]
    pub size: bool,
    /// Initial depth to expand the directory tree.
    #[arg(long, value_name = "LEVEL")]
    pub expand_level: Option<usize>,
}

/// Defines the choices for the --color option.
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ColorChoice {
    Always,
    #[default]
    Auto,
    Never,
}

/// Implements the Display trait for ColorChoice to show possible values in help messages.
impl fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}
