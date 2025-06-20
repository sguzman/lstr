//! Implements the classic, non-interactive directory tree view.

use crate::app::ViewArgs;
use crate::git;
use crate::icons;
use crate::utils;
use colored::{control, Colorize};
use ignore::{self, WalkBuilder};
use std::fs;
use std::io::{self, Write};

// Platform-specific import for unix permissions
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Executes the classic directory tree view.
pub fn run(args: &ViewArgs) -> anyhow::Result<()> {
    if !args.path.is_dir() {
        anyhow::bail!("'{}' is not a directory.", args.path.display());
    }

    let canonical_root = fs::canonicalize(&args.path)?;

    match args.color {
        crate::app::ColorChoice::Always => control::set_override(true),
        crate::app::ColorChoice::Never => control::set_override(false),
        crate::app::ColorChoice::Auto => {}
    }

    if writeln!(io::stdout(), "{}", args.path.display().to_string().blue().bold()).is_err() {
        return Ok(());
    }

    let git_repo_status = if args.git_status { git::load_status(&canonical_root)? } else { None };
    let status_cache = git_repo_status.as_ref().map(|s| &s.cache);
    let repo_root = git_repo_status.as_ref().map(|s| &s.root);

    let mut builder = WalkBuilder::new(&args.path);
    builder.hidden(!args.all).git_ignore(args.gitignore);
    if let Some(level) = args.level {
        builder.max_depth(Some(level));
    }

    let mut dir_count = 0;
    let mut file_count = 0;

    // Use the serial walker for correctness and reliability.
    for result in builder.build() {
        let entry = match result {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("lstr: ERROR: {}", err);
                continue;
            }
        };

        if entry.depth() == 0 {
            continue;
        }

        let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());
        if args.dirs_only && !is_dir {
            continue;
        }

        let git_status_str = if let (Some(cache), Some(root)) = (status_cache, repo_root) {
            if let Ok(canonical_entry) = entry.path().canonicalize() {
                if let Ok(relative_path) = canonical_entry.strip_prefix(root) {
                    cache
                        .get(relative_path)
                        .map(|s| {
                            let status_char = s.get_char();
                            let color = match s {
                                git::FileStatus::New | git::FileStatus::Renamed => {
                                    colored::Color::Green
                                }
                                git::FileStatus::Modified | git::FileStatus::Typechange => {
                                    colored::Color::Yellow
                                }
                                git::FileStatus::Deleted => colored::Color::Red,
                                git::FileStatus::Conflicted => colored::Color::BrightRed,
                                git::FileStatus::Untracked => colored::Color::Magenta,
                            };
                            format!("{} ", status_char).color(color).to_string()
                        })
                        .unwrap_or_else(|| "  ".to_string())
                } else {
                    "  ".to_string()
                }
            } else {
                "  ".to_string()
            }
        } else {
            String::new()
        };

        let metadata = if args.size || args.permissions { entry.metadata().ok() } else { None };
        let permissions_str = if args.permissions {
            let perms = if let Some(md) = &metadata {
                #[cfg(unix)]
                {
                    let mode = md.permissions().mode();
                    let file_type_char = if md.is_dir() { 'd' } else { '-' };
                    format!("{}{}", file_type_char, utils::format_permissions(mode))
                }
                #[cfg(not(unix))]
                {
                    let _ = md;
                    "----------".to_string()
                }
            } else {
                "----------".to_string()
            };
            format!("{} ", perms)
        } else {
            String::new()
        };

        let indent = "    ".repeat(entry.depth().saturating_sub(1));
        let name = entry.file_name().to_string_lossy();
        let icon_str = if args.icons {
            let (icon, color) = icons::get_icon_for_path(entry.path(), is_dir);
            format!("{} ", icon.color(color))
        } else {
            String::new()
        };
        let size_str = if args.size && !is_dir {
            metadata
                .as_ref()
                .map(|m| format!(" ({})", utils::format_size(m.len())))
                .unwrap_or_default()
        } else {
            String::new()
        };

        if is_dir {
            dir_count += 1;
            if writeln!(
                io::stdout(),
                "{}{}{}└── {}{}",
                git_status_str,
                permissions_str.dimmed(),
                indent,
                icon_str,
                name.blue().bold()
            )
            .is_err()
            {
                break;
            }
        } else {
            file_count += 1;
            if writeln!(
                io::stdout(),
                "{}{}{}└── {}{}{}",
                git_status_str,
                permissions_str.dimmed(),
                indent,
                icon_str,
                name,
                size_str.dimmed()
            )
            .is_err()
            {
                break;
            }
        }
    }

    let summary = format!("\n{} directories, {} files", dir_count, file_count);
    _ = writeln!(io::stdout(), "{}", summary);

    Ok(())
}
