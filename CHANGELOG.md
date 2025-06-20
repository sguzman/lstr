# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-XY-XY

### Removed 

- Removed the `rayon` dependency and parallel tree walking for now (related to [#10](https://github.com/bgreenwell/lstr/issues/20)).

### Added

- Added a Nix flake configuration (`flake.nix`) to provide a consistent and reproducible development environment for contributors. ([PR #10](https://github.com/bgreenwell/lstr/pull/10))

### Fixed

- Fixed a double-input bug in interactive mode on Windows that caused erratic navigation. The TUI now correctly filters for key press events. ([Closes #21](https://github.com/bgreenwell/lstr/issues/21))

- Fixed a critical issue where the classic `view` mode could produce scrambled and incorrect directory trees on certain systems, particularly on Windows. The directory walker for this mode was changed to a serial implementation to guarantee a consistent and correct output order in all environments. ([Closes #20](https://github.com/bgreenwell/lstr/issues/20))

- Optimized the release build profile in `Cargo.toml` by enabling LTO, stripping symbols, and setting `panic = "abort"`, significantly reducing the final binary size. ([PR #11](https://github.com/bgreenwell/lstr/pull/11))

- Removed the build-time dependency on `openssl` by disabling default features for the `git2` crate, which simplifies building from source.

- Refactored the project to use the version of `crossterm` re-exported by `ratatui`, preventing potential dependency version conflicts. ([PR #13](https://github.com/bgreenwell/lstr/pull/13))

- Optimized the `git2` dependency by disabling its default features. This removes the build-time requirement for `openssl` and reduces the total number of dependencies. ([PR #5](https://github.com/bgreenwell/lstr/pull/5))

- Fixed broken icons in GIF. ([Closes #4](https://github.com/bgreenwell/lstr/issues/4))

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
