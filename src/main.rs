use clap::{Parser, ValueEnum};
use colored::{control, Color, Colorize};
use ignore::{WalkBuilder, WalkState};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

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
    #[arg(long, value_name = "WHEN", default_value_t = ColorChoice::Auto, help = "Specify when to use color output")]
    color: ColorChoice,
    #[arg(short = 'L', long, help = "Maximum depth to descend")]
    level: Option<usize>,
    #[arg(short = 'd', long)]
    dirs_only: bool,
    #[arg(short = 'a', long, help = "Show all files, including hidden ones")]
    all: bool,
    #[arg(
        short = 'g',
        long,
        help = "Respect .gitignore and other standard ignore files"
    )]
    gitignore: bool,
    #[arg(long, help = "Display file-specific icons (requires a Nerd Font)")]
    icons: bool,
}

/// Returns a Nerd Font icon and a color for a given path.
fn get_icon_for_path(path: &Path, is_dir: bool) -> (String, Color) {
    if is_dir {
        return ("".to_string(), Color::Blue); // Folder icon
    }

    let icon = match path.file_name().and_then(|s| s.to_str()) {
        //Some("Cargo.toml") | Some("Cargo.lock") => "",
        Some("Cargo.toml") => "",
        Some("Cargo.lock") => "",
        Some(".gitignore") | Some(".gitattributes") => "",
        Some("LICENSE") => "",
        Some("README.md") => "",
        Some("Dockerfile") => "",
        Some("Makefile") | Some("makefile") => "",
        Some("CMakeLists.txt") => "",
        _ => {
            // Fall back to matching file extensions if no special filename matches
            match path.extension().and_then(|s| s.to_str()) {
                // Documents and text
                Some("md") => "",
                Some("txt") => "",
                Some("pdf") => "",

                // Programming and scripting languages
                Some("rs") => "",                                          // Rust
                Some("r") | Some("R") => "",                               // R
                Some("py") => "",                                          // Python
                Some("js") => "",                                          // JavaScript
                Some("ts") | Some("tsx") => "",                            // TypeScript
                Some("java") => "",                                        // Java
                Some("kt") | Some("kts") => "",                            // Kotlin
                Some("swift") => "",                                       // Swift
                Some("go") => "",                                          // Go
                Some("php") => "",                                         // PHP
                Some("rb") => "",                                          // Ruby
                Some("c") | Some("h") => "",                               // C
                Some("cpp") | Some("hpp") | Some("cc") | Some("hh") => "", // C++
                Some("cs") => "󰌛",                                          // C#
                Some("sh") | Some("bash") | Some("zsh") => "",             // Shell
                Some("asm") | Some("s") => "",                             // Assembly
                Some("wasm") => "",                                        // WebAssembly

                // Web
                Some("html") => "",
                Some("css") | Some("scss") => "",
                Some("svg") => "󰜡", // SVG icon

                // Config, data, and lock files
                Some("toml") => "",
                Some("json") => "",
                Some("yaml") | Some("yml") => "󰗊",
                Some("xml") => "󰗀",
                Some("env") => "",
                Some("sql") | Some("db") | Some("sqlite3") => "",
                Some("csv") => "",
                Some("lock") => "",   // Generic lock file
                Some("gradle") => "", // Gradle/Android
                Some("tf") => "",     // Terraform

                // Archives
                Some("zip") | Some("gz") | Some("tar") | Some("rar") => "",

                // Media and fonts
                Some("png") | Some("jpg") | Some("jpeg") | Some("gif") => "",
                Some("mp3") | Some("flac") | Some("wav") => "",
                Some("mp4") | Some("mov") | Some("mkv") => "",
                Some("ttf") | Some("otf") | Some("woff") | Some("woff2") => "",

                _ => "", // Default file icon
            }
        }
    };

    // Determine color based on the icon
    let color = match icon {
        // Languages
        "" | "" => Color::Red,                          // Rust, Java
        "" | "" | "" | "" | "" | "" => Color::Blue, // R, C, C++, TS, Docker, Terraform
        "" | "" => Color::Yellow,                       // Python, JS
        "" | "" | "" => Color::BrightRed,              // Swift, PHP, Ruby
        "" | "" => Color::Green,                        // Go, Shell
        "󰌛" | "" => Color::Magenta,                      // C#, Kotlin

        // Config and data
        "" | "󰗊" | "" => Color::BrightYellow,
        "" => Color::Yellow,
        "" | "" => Color::Cyan,

        // Other
        "" => Color::BrightBlack,         // Git
        "" | "" | "" => Color::Magenta, // Media
        "" => Color::BrightRed,           // Archives
        _ => Color::White,                 // Default color for other icons
    };

    (icon.to_string(), color)
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

    let mut builder = WalkBuilder::new(&args.path);
    builder
        .hidden(!args.all)
        .git_ignore(args.gitignore)
        .max_depth(args.level);

    let walker = builder.build_parallel();
    let dir_count = AtomicU32::new(0);
    let file_count = AtomicU32::new(0);

    walker.run(|| {
        Box::new(|result| {
            let entry = match result {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("lstr: ERROR: {}", err);
                    return WalkState::Continue;
                }
            };

            if entry.depth() == 0 {
                return WalkState::Continue;
            }

            let is_dir = entry.file_type().map_or(false, |ft| ft.is_dir());
            if args.dirs_only && !is_dir {
                return WalkState::Continue;
            }

            let indent = "    ".repeat(entry.depth().saturating_sub(1));
            let name = entry.file_name().to_string_lossy();

            let icon_str = if args.icons {
                let (icon, color) = get_icon_for_path(entry.path(), is_dir);
                format!("{} ", icon.color(color)) // Add a space after the icon
            } else {
                String::new() // If --icons is not used, the "icon" is an empty string
            };

            if is_dir {
                dir_count.fetch_add(1, Ordering::Relaxed);
                println!("{}└── {}{}", indent, icon_str, name.blue());
            } else {
                file_count.fetch_add(1, Ordering::Relaxed);
                println!("{}└── {}{}", indent, icon_str, name);
            }

            WalkState::Continue
        })
    });

    println!(
        "\n{} directories, {} files",
        dir_count.load(Ordering::Relaxed),
        file_count.load(Ordering::Relaxed)
    );
}
