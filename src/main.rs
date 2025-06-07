use clap::{Parser, ValueEnum};
use std::fmt;
use std::path::{PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use colored::{control, Colorize};
use ignore::{WalkBuilder, WalkState};

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
    #[arg(short = 'L', long, help = "Maximum depth to descend")]
    level: Option<usize>,
    #[arg(short = 'd', long)]
    dirs_only: bool,
    #[arg(short = 'a', long, help = "Show all files, including hidden ones")]
    all: bool,
    #[arg(short = 'g', long, help = "Respect .gitignore files (enabled by default)")]
    gitignore: bool,
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

    // --- Configure the WalkBuilder ---
    let mut builder = WalkBuilder::new(&args.path);
    builder
        .hidden(!args.all) // `hidden(true)` is the default, so we invert the logic
        .git_ignore(args.gitignore)
        .max_depth(args.level);

    let walker = builder.build_parallel();

    // Use Atomic counters for safe parallel counting
    let dir_count = AtomicU32::new(0);
    let file_count = AtomicU32::new(0);

    // --- Let the `ignore` crate do all the work ---
    walker.run(|| {
        Box::new(|result| {
            let entry = match result {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("lstr: ERROR: {}", err);
                    return WalkState::Continue;
                }
            };

            // The first result is the root directory itself, which we've already printed.
            if entry.depth() == 0 {
                return WalkState::Continue;
            }

            if args.dirs_only && !entry.file_type().map_or(false, |ft| ft.is_dir()) {
                return WalkState::Continue;
            }

            // Generate the prefix based on the entry's depth
            let indent = "    ".repeat(entry.depth().saturating_sub(1));
            let name = entry.file_name().to_string_lossy();
            
            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                dir_count.fetch_add(1, Ordering::Relaxed);
                println!("{}└── {}", indent, name.blue());
            } else {
                file_count.fetch_add(1, Ordering::Relaxed);
                println!("{}└── {}", indent, name);
            }

            WalkState::Continue
        })
    });

    println!("\n{} directories, {} files", dir_count.load(Ordering::Relaxed), file_count.load(Ordering::Relaxed));
}