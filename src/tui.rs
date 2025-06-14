//! Implements the interactive terminal user interface (TUI) mode.
//!
//! This module contains all logic for running `lstr` in an interactive
//! session, including state management, event handling, and rendering.

use crate::app::InteractiveArgs;
use crate::icons;
use crate::utils;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ignore::WalkBuilder;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
    Frame, Terminal,
};
use std::env;
use std::io::{self, Stdout, Write};
use std::path::PathBuf;
use std::process::Command;

// Platform-specific import for unix permissions
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// An action to be performed after the TUI exits.
enum PostExitAction {
    None,
    OpenFile(PathBuf),
}

/// A single entry in our file tree, holding its path and state.
#[derive(Debug, Clone)]
struct FileEntry {
    path: PathBuf,
    depth: usize,
    is_dir: bool,
    is_expanded: bool,
    /// The size of the file in bytes, if it is a file.
    size: Option<u64>,
    /// The formatted permissions string, if requested.
    permissions: Option<String>,
}

/// Holds the state of the interactive application.
struct AppState {
    master_entries: Vec<FileEntry>,
    visible_entries: Vec<FileEntry>,
    list_state: ListState,
}

impl AppState {
    /// Creates a new AppState, performing the initial scan and setup.
    fn new(args: &InteractiveArgs) -> anyhow::Result<Self> {
        // Pass all relevant flags down to the scanning function.
        let mut master_entries =
            scan_directory(&args.path, args.gitignore, args.all, args.size, args.permissions)?;

        if let Some(expand_level) = args.expand_level {
            for entry in &mut master_entries {
                if entry.is_dir && entry.depth < expand_level {
                    entry.is_expanded = true;
                }
            }
        }

        let mut app_state =
            Self { master_entries, visible_entries: Vec::new(), list_state: ListState::default() };

        app_state.regenerate_visible_entries();

        if !app_state.visible_entries.is_empty() {
            app_state.list_state.select(Some(0));
        }

        Ok(app_state)
    }

    /// Regenerates the `visible_entries` list from the `master_entries` list.
    fn regenerate_visible_entries(&mut self) {
        self.visible_entries.clear();
        let mut parent_expanded_stack: Vec<bool> = Vec::new();

        for entry in &self.master_entries {
            while parent_expanded_stack.len() >= entry.depth {
                parent_expanded_stack.pop();
            }

            if parent_expanded_stack.iter().all(|&x| x) {
                self.visible_entries.push(entry.clone());
            }

            if entry.is_dir {
                parent_expanded_stack.push(entry.is_expanded);
            }
        }
    }

    /// Moves the selection to the next item in the list.
    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.visible_entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Moves the selection to the previous item in the list.
    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.visible_entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Gets the currently selected entry, if any.
    fn get_selected_entry(&self) -> Option<&FileEntry> {
        self.list_state.selected().and_then(|i| self.visible_entries.get(i))
    }

    /// Toggles the expansion state of the currently selected directory.
    fn toggle_selected_directory(&mut self) {
        if let Some(selected_index) = self.list_state.selected() {
            let selected_path = self.visible_entries[selected_index].path.clone();

            if let Some(master_entry) =
                self.master_entries.iter_mut().find(|e| e.path == selected_path)
            {
                if master_entry.is_dir {
                    master_entry.is_expanded = !master_entry.is_expanded;
                }
            }

            self.regenerate_visible_entries();

            if let Some(new_index) =
                self.visible_entries.iter().position(|e| e.path == selected_path)
            {
                self.list_state.select(Some(new_index));
            } else {
                let new_selection = selected_index.min(self.visible_entries.len() - 1);
                self.list_state.select(Some(new_selection));
            }
        }
    }
}

/// Executes the interactive TUI mode.
pub fn run(args: &InteractiveArgs) -> anyhow::Result<()> {
    let mut app_state = AppState::new(args)?;
    let mut terminal = setup_terminal()?;

    let post_exit_action = run_app(&mut terminal, &mut app_state, args)?;

    // Restore the terminal *before* performing the final action.
    restore_terminal(&mut terminal)?;

    // Perform the action after the TUI has been shut down.
    if let PostExitAction::OpenFile(path) = post_exit_action {
        let editor = env::var("EDITOR").unwrap_or_else(|_| {
            if cfg!(windows) {
                "notepad".to_string()
            } else {
                "vim".to_string()
            }
        });
        Command::new(editor).arg(path).status()?;
    }

    Ok(())
}

/// The main application loop.
fn run_app<B: Backend + Write>(
    terminal: &mut Terminal<B>,
    app_state: &mut AppState,
    args: &InteractiveArgs,
) -> anyhow::Result<PostExitAction> {
    loop {
        terminal.draw(|f| ui(f, app_state, args))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break Ok(PostExitAction::None),
                KeyCode::Down | KeyCode::Char('j') => app_state.next(),
                KeyCode::Up | KeyCode::Char('k') => app_state.previous(),
                KeyCode::Enter => {
                    if let Some(entry) = app_state.get_selected_entry() {
                        if entry.is_dir {
                            app_state.toggle_selected_directory();
                        } else {
                            // Break the loop and return the path to open.
                            break Ok(PostExitAction::OpenFile(entry.path.clone()));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// Renders the user interface.
fn ui(f: &mut Frame, app_state: &mut AppState, args: &InteractiveArgs) {
    let frame_width = f.size().width as usize;

    let items: Vec<ListItem> = app_state
        .visible_entries
        .iter()
        .map(|entry| {
            let mut spans = Vec::new();

            // Permissions
            if args.permissions {
                let perms_str = entry.permissions.as_deref().unwrap_or("----------");
                spans.push(Span::styled(
                    format!("{perms_str} "),
                    Style::default().fg(Color::DarkGray),
                ));
            }

            // Tree branch and indentation
            let indent_str = "    ".repeat(entry.depth.saturating_sub(1));
            spans.push(Span::raw(indent_str));
            let branch_str = if entry.is_dir {
                if entry.is_expanded {
                    "▼ "
                } else {
                    "▶ "
                }
            } else {
                "  "
            };
            spans.push(Span::raw(branch_str));

            // Icon
            if args.icons {
                let (icon, color) = icons::get_icon_for_path(&entry.path, entry.is_dir);
                spans.push(Span::styled(
                    format!("{} ", icon),
                    Style::default().fg(map_color(color)),
                ));
            }

            // Name
            let name = entry.path.file_name().unwrap().to_string_lossy();
            let mut name_span = Span::raw(name.to_string());

            // Style directory names
            if entry.is_dir {
                let dir_style = Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD);
                for span in &mut spans {
                    span.style = span.style.patch(dir_style);
                }
                name_span.style = name_span.style.patch(dir_style);
            }
            spans.push(name_span);

            // Right side: file size (if enabled)
            if args.size {
                if let Some(size) = entry.size {
                    let size_str = utils::format_size(size);

                    // Calculate padding to right-align the size
                    let left_len: usize = spans.iter().map(|s| s.width()).sum();
                    let padding =
                        frame_width.saturating_sub(left_len).saturating_sub(size_str.len());

                    spans.push(Span::raw(" ".repeat(padding)));
                    spans.push(Span::styled(size_str, Style::default().fg(Color::DarkGray)));
                }
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items)
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, f.size(), &mut app_state.list_state);
}

/// Scans the directory and collects file entries.
fn scan_directory(
    path: &PathBuf,
    gitignore: bool,
    all: bool,
    with_size: bool,
    with_permissions: bool,
) -> anyhow::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    let mut builder = WalkBuilder::new(path);
    builder.hidden(!all).git_ignore(gitignore);
    for entry in builder.build().flatten() {
        if entry.depth() == 0 {
            continue;
        }

        let metadata = if with_size || with_permissions { entry.metadata().ok() } else { None };
        let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());

        let size = if with_size && !is_dir { metadata.as_ref().map(|m| m.len()) } else { None };

        let permissions = if with_permissions {
            metadata.map(|md| {
                #[cfg(unix)]
                {
                    let mode = md.permissions().mode();
                    let file_type_char = if md.is_dir() { 'd' } else { '-' };
                    format!("{}{}", file_type_char, utils::format_permissions(mode))
                }
                #[cfg(not(unix))]
                {
                    "----------".to_string()
                }
            })
        } else {
            None
        };

        entries.push(FileEntry {
            path: entry.path().to_path_buf(),
            depth: entry.depth(),
            is_dir,
            is_expanded: false,
            size,
            permissions,
        });
    }
    Ok(entries)
}

/// Maps a `colored::Color` to a `ratatui::style::Color`.
fn map_color(c: colored::Color) -> Color {
    match c {
        colored::Color::Black => Color::Black,
        colored::Color::Red => Color::Red,
        colored::Color::Green => Color::Green,
        colored::Color::Yellow => Color::Yellow,
        colored::Color::Blue => Color::Blue,
        colored::Color::Magenta => Color::Magenta,
        colored::Color::Cyan => Color::Cyan,
        colored::Color::White => Color::White,
        colored::Color::BrightBlack => Color::Gray,
        colored::Color::BrightRed => Color::LightRed,
        colored::Color::BrightGreen => Color::LightGreen,
        colored::Color::BrightYellow => Color::LightYellow,
        colored::Color::BrightBlue => Color::LightBlue,
        colored::Color::BrightMagenta => Color::LightMagenta,
        colored::Color::BrightCyan => Color::LightCyan,
        colored::Color::TrueColor { r, g, b } => Color::Rgb(r, g, b),
        _ => Color::Reset,
    }
}

// --- Terminal setup and restore functions ---
fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(anyhow::Error::from)
}

fn restore_terminal<B: Backend + Write>(terminal: &mut Terminal<B>) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

// Unit tests for the TUI AppState logic
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a mock AppState for testing.
    fn setup_test_app_state() -> AppState {
        let master_entries = vec![
            FileEntry {
                path: PathBuf::from("src"),
                depth: 1,
                is_dir: true,
                is_expanded: false,
                size: None,
                permissions: Some("drwxr-xr-x".to_string()),
            },
            FileEntry {
                path: PathBuf::from("src/main.rs"),
                depth: 2,
                is_dir: false,
                is_expanded: false,
                size: Some(1024),
                permissions: Some("-rw-r--r--".to_string()),
            },
            FileEntry {
                path: PathBuf::from("README.md"),
                depth: 1,
                is_dir: false,
                is_expanded: false,
                size: Some(512),
                permissions: Some("-rw-r--r--".to_string()),
            },
        ];

        let mut app_state = AppState {
            master_entries,
            visible_entries: Vec::new(),
            list_state: ListState::default(),
        };

        app_state.regenerate_visible_entries();
        app_state.list_state.select(Some(0));
        app_state
    }

    #[test]
    fn test_navigation() {
        let mut app_state = setup_test_app_state();
        assert_eq!(app_state.list_state.selected(), Some(0));
        app_state.next();
        assert_eq!(app_state.list_state.selected(), Some(1));
        app_state.next();
        assert_eq!(app_state.list_state.selected(), Some(0));
        app_state.previous();
        assert_eq!(app_state.list_state.selected(), Some(1));
        app_state.previous();
        assert_eq!(app_state.list_state.selected(), Some(0));
    }

    #[test]
    fn test_toggle_directory() {
        let mut app_state = setup_test_app_state();
        assert_eq!(app_state.visible_entries.len(), 2);
        app_state.list_state.select(Some(0));
        app_state.toggle_selected_directory();
        assert_eq!(app_state.visible_entries.len(), 3);
        assert_eq!(app_state.visible_entries[1].path, PathBuf::from("src/main.rs"));
        app_state.toggle_selected_directory();
        assert_eq!(app_state.visible_entries.len(), 2);
    }

    #[test]
    fn test_get_selected_entry() {
        let mut app_state = setup_test_app_state();
        app_state.list_state.select(Some(1));
        let selected = app_state.get_selected_entry();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().path, PathBuf::from("README.md"));
    }
}
