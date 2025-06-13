//! lstr: A blazingly fast, minimalist directory tree viewer.
//!
//! This is the main entry point for the lstr application. It handles parsing
//! command-line arguments and dispatching to the appropriate command handler.

// Declare the modules that make up the application.
mod app;
mod icons;
mod tui;
mod view;

use app::{Args, Commands};
use clap::Parser;

/// The main function and entry point of the application.
///
/// It parses command-line arguments and executes the corresponding command.
/// If no subcommand is given, it defaults to the classic tree `view`.
///
/// # Returns
///
/// * `Ok(())` on successful execution.
/// * `Err(anyhow::Error)` if any error occurs during execution.
fn main() -> anyhow::Result<()> {
    // Parse the command-line arguments into our Args struct.
    let args = Args::parse();

    // Check if a subcommand was passed. If not, default to the `view` command.
    match &args.command {
        Some(Commands::Interactive(interactive_args)) => tui::run(interactive_args),
        None => view::run(&args.view),
    }
}
