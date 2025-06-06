use clap::{Parser, ValueEnum};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use colored::{control, Colorize};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorChoice {
    Always,
    Auto,
    Never,
}

impl fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,
    #[arg(
        long,
        value_name = "WHEN",
        default_value_t = ColorChoice::Auto,
        help = "Specify when to use color output"
    )]
    color: ColorChoice,
    #[arg(short = 'L', long, value_name = "LEVEL", default_value_t = usize::MAX)]
    level: usize,
    #[arg(short = 'd', long)]
    dirs_only: bool,
    #[arg(short = 'a', long)]
    all: bool,
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

    let mut dir_count = 0;
    let mut file_count = 0;

    walk(
        &args.path,
        0,
        "",
        &args,
        &mut dir_count,
        &mut file_count,
    );

    println!("\n{} directories, {} files", dir_count, file_count);
}

fn walk(
    dir: &Path,
    depth: usize,
    prefix: &str,
    args: &Args,
    dir_count: &mut u32,
    file_count: &mut u32,
) {
    if depth >= args.level {
        return;
    }
    
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            if e.kind() != std::io::ErrorKind::PermissionDenied {
                 eprintln!("lstr: Error reading '{}': {}", dir.display(), e);
            }
            return;
        }
    };
    
    let mut entries: Vec<_> = entries
        .filter_map(Result::ok)
        .filter(|e| args.all || !e.file_name().to_string_lossy().starts_with('.'))
        .collect();

    entries.sort_by_key(|e| e.file_name());

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

        if path.is_dir() {
            *dir_count += 1;
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            walk(&path, depth + 1, &new_prefix, args, dir_count, file_count);
        } else {
            *file_count += 1;
        }
    }
}
