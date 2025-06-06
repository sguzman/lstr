// =============================================================================
// lstr - A blazingly fast, minimalist directory tree viewer
// src/main.rs
//
// This is the complete and final version of the core logic, incorporating
// all features developed.
// =============================================================================

use clap::{Parser, ValueEnum};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use colored::{control, Colorize};

// -----------------------------------------------------------------------------
// Enum for Command-Line Argument Choices
// -----------------------------------------------------------------------------

/// Defines the choices for the --color flag.
/// Deriving `ValueEnum` lets clap parse it from a string.
/// Deriving `Copy` and `Clone` allows it to be used with `default_value_t`.
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorChoice {
    /// Always use color output, even when piping to a file.
    Always,
    /// Use color only when printing to a terminal (default).
    Auto,
    /// Never use color output.
    Never,
}

/// Implements the `Display` trait so clap can print the default value in help messages.
/// This implementation satisfies the trait bound error from the previous step.
impl fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the `to_possible_value` method provided by `ValueEnum`
        // to get the string representation (e.g., "auto").
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}


// -----------------------------------------------------------------------------
// Command-Line Argument Struct
// -----------------------------------------------------------------------------

/// lstr (LiSt-TRree) walks a directory and prints its contents in a tree structure.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory path to list. Defaults to the current directory.
    #[arg(default_value = ".")]
    path: PathBuf,

    /// When to use color output.
    #[arg(
        long,
        value_name = "WHEN",
        default_value_t = ColorChoice::Auto,
        help = "Specify when to use color output"
    )]
    color: ColorChoice,

    /// Maximum depth to descend.
    #[arg(short = 'L', long, value_name = "LEVEL", default_value_t = usize::MAX)]
    level: usize,

    /// List directories only.
    #[arg(short = 'd', long)]
    dirs_only: bool,

    /// List all files, including hidden ones (those starting with a dot).
    #[arg(short = 'a', long)]
    all: bool,
}


// -----------------------------------------------------------------------------
// Main Application Entry Point
// -----------------------------------------------------------------------------

fn main() {
    let args = Args::parse();

    // Configure global color output based on the CLI argument.
    // This must be done right after parsing args.
    match args.color {
        ColorChoice::Always => control::set_override(true),
        ColorChoice::Never => control::set_override(false),
        // If "Auto", we do nothing. `colored` will detect the terminal automatically.
        ColorChoice::Auto => {}
    }

    // Validate that the path is a directory.
    if !args.path.is_dir() {
        eprintln!("lstr: Error: '{}' is not a directory.", args.path.display());
        std::process::exit(1);
    }
    
    // Print the root directory, colored for visibility.
    println!("{}", args.path.display().to_string().blue());

    // Initialize counters for the final summary.
    let mut dir_count = 0;
    let mut file_count = 0;

    // Start the recursive walk.
    walk(
        &args.path,
        0,
        "",
        &args,
        &mut dir_count,
        &mut file_count,
    );

    // Print the final summary.
    println!("\n{} directories, {} files", dir_count, file_count);
}


// -----------------------------------------------------------------------------
// Recursive Directory Walker Function
// -----------------------------------------------------------------------------

/// Recursively walks a directory and prints its contents in a tree structure.
fn walk(
    dir: &Path,
    depth: usize,
    prefix: &str,
    args: &Args,
    dir_count: &mut u32,
    file_count: &mut u32,
) {
    // Stop recursion if the specified depth is reached.
    if depth >= args.level {
        return;
    }
    
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            // Silently ignore permission errors for a cleaner output.
            if e.kind() != std::io::ErrorKind::PermissionDenied {
                 eprintln!("lstr: Error reading '{}': {}", dir.display(), e);
            }
            return;
        }
    };
    
    // Collect, filter, and sort entries before processing.
    let mut entries: Vec<_> = entries
        .filter_map(Result::ok)
        .filter(|e| args.all || !e.file_name().to_string_lossy().starts_with('.'))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    // Use a peekable iterator to determine if an entry is the last one.
    let mut iter = entries.iter().peekable();

    while let Some(entry) = iter.next() {
        let path = entry.path();
        
        // Skip files if the `dirs_only` flag is active.
        if args.dirs_only && !path.is_dir() {
            continue; 
        }

        let is_last = iter.peek().is_none();
        let connector = if is_last { "└──" } else { "├──" };
        let file_name = entry.file_name();

        if path.is_dir() {
            // Print directories in blue.
            println!(
                "{} {} {}",
                prefix,
                connector,
                file_name.to_string_lossy().blue()
            );

            *dir_count += 1;
            let new_prefix = if is_last {
                format!("{}    ", prefix) // No vertical line for the last entry.
            } else {
                format!("{}│   ", prefix) // Continue the vertical line.
            };
            // Recursive call for the subdirectory.
            walk(&path, depth + 1, &new_prefix, args, dir_count, file_count);
        } else {
            // Print files in the default terminal color.
            println!(
                "{} {} {}",
                prefix,
                connector,
                file_name.to_string_lossy()
            );
            *file_count += 1;
        }
    }
}