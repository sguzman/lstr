use clap::{Parser, ValueEnum};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use colored::{control, Colorize};
use rayon::prelude::*;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorChoice { Always, Auto, Never }

impl fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,
    #[arg(long, value_name = "WHEN", default_value_t = ColorChoice::Auto, help = "Specify when to use color output")]
    color: ColorChoice,
    #[arg(short = 'L', long, value_name = "LEVEL", default_value_t = usize::MAX)]
    level: usize,
    #[arg(short = 'd', long)]
    dirs_only: bool,
    #[arg(short = 'a', long)]
    all: bool,
    #[arg(long, help = "Run in single-threaded (serial) mode")]
    serial: bool,
}

fn main() {
    let args = Args::parse();
    match args.color {
        ColorChoice::Always => control::set_override(true),
        ColorChoice::Never => control::set_override(false),
        ColorChoice::Auto => {}
    }

    if !args.path.is_dir() {
        eprintln!("lstr: Error: '{}' is not a directory.", args.path.display());
        std::process::exit(1);
    }
    
    println!("{}", args.path.display().to_string().blue());

    let (dir_count, file_count) = walk(&args.path, "", &args);

    println!("\n{} directories, {} files", dir_count, file_count);
}

/// Main recursive function that dispatches to parallel or serial workers.
fn walk(dir: &Path, prefix: &str, args: &Args) -> (u32, u32) {
    // Base case: Stop recursion if we are at the desired level depth. The check 
    // is `prefix.chars().count() / 4 >= args.level` because each level adds 4 
    // characters to the prefix.
    if prefix.chars().count() / 4 >= args.level {
        return (0, 0);
    }

    let Ok(read_dir) = fs::read_dir(dir) else { return (0, 0) };
    
    let mut entries: Vec<_> = read_dir
        .filter_map(Result::ok)
        .filter(|e| args.all || !e.file_name().to_string_lossy().starts_with('.'))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    // Print loop (always serial). This loop iterates through all entries at the 
    // current level and prints them. This *must* be sequential to maintain the 
    // correct sorted output.
    let mut iter = entries.iter().peekable();
    while let Some(entry) = iter.next() {
        let path = entry.path();
        
        if args.dirs_only && !path.is_dir() {
            continue;
        }

        let is_last = iter.peek().is_none();
        let connector = if is_last { "└── " } else { "├── " };
        
        if path.is_dir() {
            println!("{}{}{}", prefix, connector, entry.file_name().to_string_lossy().blue());
        } else {
            println!("{}{}{}", prefix, connector, entry.file_name().to_string_lossy());
        }
    }

    // Recursive work (parallel or serial). This is where we kick off the 
    // expensive work for the subdirectories found above.
    let total_entry_count = entries.len(); 
    let dir_entries: Vec<_> = entries.into_iter().filter(|e| e.path().is_dir()).collect(); 
    let file_count = total_entry_count as u32 - dir_entries.len() as u32; 



    let counts: (u32, u32) = if args.serial {
        // Serial recursion 
        dir_entries.iter().enumerate().map(|(i, entry)| {
            let is_last = i == dir_entries.len() - 1;
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            walk(&entry.path(), &new_prefix, args)
        }).fold((0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1))
    } else {
        // Parallel recursion using Rayon
        dir_entries.par_iter().enumerate().map(|(i, entry)| {
            let is_last = i == dir_entries.len() - 1;
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            walk(&entry.path(), &new_prefix, args)
        }).reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
    };

    // Return the total counts: sub-directory counts + this level's counts.
    (counts.0 + dir_entries.len() as u32, counts.1 + file_count)
}