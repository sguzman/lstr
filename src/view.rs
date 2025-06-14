//! Implements the classic, non-interactive directory tree view.

use crate::app::ViewArgs;
use crate::git;
use crate::icons;
use crate::utils;
use colored::{control, Colorize};
use ignore::{WalkBuilder, WalkState};
use std::fs;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

// Platform-specific import for unix permissions
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Executes the classic directory tree view.
pub fn run(args: &ViewArgs) -> anyhow::Result<()> {
    // Check if the path is a directory BEFORE trying to canonicalize it.
    if !args.path.is_dir() {
        anyhow::bail!("'{}' is not a directory.", args.path.display());
    }

    // Now that we know the path is valid, canonicalize it to resolve symlinks.
    let root_path = Arc::new(fs::canonicalize(&args.path)?);

    match args.color {
        crate::app::ColorChoice::Always => control::set_override(true),
        crate::app::ColorChoice::Never => control::set_override(false),
        crate::app::ColorChoice::Auto => {}
    }

    if writeln!(io::stdout(), "{}", root_path.display().to_string().blue().bold()).is_err() {
        return Ok(()); // Exit silently on broken pipe
    }

    let git_repo_status = if args.git_status { git::load_status(&root_path)? } else { None };

    let mut builder = WalkBuilder::new(root_path.as_ref());
    builder.hidden(!args.all).git_ignore(args.gitignore).max_depth(args.level);

    let walker = builder.build_parallel();
    let dir_count = Arc::new(AtomicU32::new(0));
    let file_count = Arc::new(AtomicU32::new(0));

    walker.run({
        let dir_count = Arc::clone(&dir_count);
        let file_count = Arc::clone(&file_count);
        let git_repo_status = git_repo_status.clone();
        let root_path = Arc::clone(&root_path);

        move || {
            let dir_count = Arc::clone(&dir_count);
            let file_count = Arc::clone(&file_count);
            let git_repo_status = git_repo_status.clone();
            let root_path = Arc::clone(&root_path);

            Box::new(move |result| {
                let status_cache = git_repo_status.as_ref().map(|s| &s.cache);
                let repo_root = git_repo_status.as_ref().map(|s| &s.root);

                let entry = match result {
                    Ok(entry) => entry,
                    Err(err) => {
                        eprintln!("lstr: ERROR: {}", err);
                        return WalkState::Continue;
                    }
                };

                if entry.path() == root_path.as_ref() {
                    return WalkState::Continue;
                }

                let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());
                if args.dirs_only && !is_dir {
                    return WalkState::Continue;
                }

                let git_status_str = if let (Some(cache), Some(root)) = (status_cache, repo_root) {
                    if let Ok(relative_path) = entry.path().strip_prefix(root) {
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
                    String::new()
                };

                let metadata =
                    if args.size || args.permissions { entry.metadata().ok() } else { None };

                let permissions_str = if args.permissions {
                    let mut perms_string = "----------".to_string();
                    if let Some(md) = &metadata {
                        #[cfg(unix)]
                        {
                            let mode = md.permissions().mode();
                            let file_type_char = if md.is_dir() { 'd' } else { '-' };
                            perms_string =
                                format!("{}{}", file_type_char, utils::format_permissions(mode));
                        }
                    }
                    format!("{} ", perms_string)
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
                    dir_count.fetch_add(1, Ordering::Relaxed);
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
                        return WalkState::Quit;
                    }
                } else {
                    file_count.fetch_add(1, Ordering::Relaxed);
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
                        return WalkState::Quit;
                    }
                }

                WalkState::Continue
            })
        }
    });

    let summary = format!(
        "\n{} directories, {} files",
        dir_count.load(Ordering::Relaxed),
        file_count.load(Ordering::Relaxed)
    );
    _ = writeln!(io::stdout(), "{}", summary);

    Ok(())
}
