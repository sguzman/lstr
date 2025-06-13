//! Implements the classic, non-interactive directory tree view.

use crate::app::ViewArgs;
use crate::icons;
use crate::utils; // Import our new utils module
use colored::{control, Colorize};
use ignore::{WalkBuilder, WalkState};
use std::io::{self, Write};
use std::sync::atomic::{AtomicU32, Ordering};

/// Executes the classic directory tree view.
pub fn run(args: &ViewArgs) -> anyhow::Result<()> {
    if !args.path.is_dir() {
        anyhow::bail!("'{}' is not a directory.", args.path.display());
    }

    match args.color {
        crate::app::ColorChoice::Always => control::set_override(true),
        crate::app::ColorChoice::Never => control::set_override(false),
        crate::app::ColorChoice::Auto => {}
    }

    if writeln!(io::stdout(), "{}", args.path.display().to_string().blue().bold()).is_err() {
        return Ok(()); // Exit silently on broken pipe
    }

    let mut builder = WalkBuilder::new(&args.path);
    builder.hidden(!args.all).git_ignore(args.gitignore).max_depth(args.level);

    let walker = builder.build_parallel();
    let dir_count = AtomicU32::new(0);
    let file_count = AtomicU32::new(0);

    walker.run(|| {
        macro_rules! write_line {
            ($($arg:tt)*) => {
                if writeln!(io::stdout(), $($arg)*).is_err() {
                    return WalkState::Quit;
                }
            };
        }

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

            let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());
            if args.dirs_only && !is_dir {
                return WalkState::Continue;
            }

            let indent = "    ".repeat(entry.depth().saturating_sub(1));
            let name = entry.file_name().to_string_lossy();

            let icon_str = if args.icons {
                let (icon, color) = icons::get_icon_for_path(entry.path(), is_dir);
                format!("{} ", icon.color(color))
            } else {
                String::new()
            };

            // If --size is passed, get the file size and format it.
            let size_str = if args.size && !is_dir {
                entry
                    .metadata()
                    .ok()
                    .map(|m| format!(" ({})", utils::format_size(m.len())))
                    .unwrap_or_default()
            } else {
                String::new()
            };

            if is_dir {
                dir_count.fetch_add(1, Ordering::Relaxed);
                write_line!("{}└── {}{}", indent, icon_str, name.blue().bold());
            } else {
                file_count.fetch_add(1, Ordering::Relaxed);
                // Append the size string to the output for files.
                write_line!("{}└── {}{}{}", indent, icon_str, name, size_str.dimmed());
            }

            WalkState::Continue
        })
    });

    let summary = format!(
        "\n{} directories, {} files",
        dir_count.load(Ordering::Relaxed),
        file_count.load(Ordering::Relaxed)
    );
    _ = writeln!(io::stdout(), "{}", summary);

    Ok(())
}
