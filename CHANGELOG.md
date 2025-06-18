# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-06-17

### Added

-   **Interactive Mode:** A new `interactive` subcommand that launches a terminal-based UI.
    -   Keyboard-driven navigation (`Up`/`Down`, `j`/`k`).
    -   Directory expansion and collapsing with `Enter`.
    -   Ability to open selected files in the default editor (`$EDITOR`).
-   **Git Integration:** A new `-G, --git-status` flag displays file statuses (`M`, `A`, `?`, etc.) in both classic and interactive modes.
-   **Shell Integration:** In interactive mode, pressing `Ctrl+s` now quits and prints the selected path to `stdout`, allowing `lstr` to be used as a file picker for other shell commands.
-   **Rich Information Display:**
    -   Added `--icons` flag to display file-specific icons (requires a Nerd Font).
    -   Added `-s, --size` flag to display file sizes.
    -   Added `-p, --permissions` flag to display file permissions (Unix-like systems only).

### Fixed

-   Resolved an issue where the `--gitignore` (`-g`) flag would fail to ignore files in certain environments.
-   Fixed a critical bug where the interactive TUI would hang and produce garbled output when piped to another command.

## [0.1.1] - 2025-06-06

### Added

- Initial release of `lstr`.
- Core recursive directory tree walking and printing functionality.
- Colorized output for directories, configurable with the `--color` flag (`always`, `auto`, `never`).
- Control over recursion depth via the `-L` flag.
- Option to display directories only via the `-d` flag.
- Option to show hidden files and directories via the `-a` flag.
- Ability to respect `.gitignore` and other standard ignore files via the `-g` flag.

## [0.1.0] - 2025-06-06

- Initial release.
