# lstr

[![Build Status](https://github.com/bgreenwell/lstr/actions/workflows/ci.yml/badge.svg)](https://github.com/bgreenwell/lstr/actions)
[![Latest Version](https://img.shields.io/crates/v/lstr.svg)](https://crates.io/crates/lstr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A blazingly fast, minimalist directory tree viewer, written in Rust. Inspired by the command line program [tree](https://github.com/Old-Man-Programmer/tree), with a powerful interactive mode.

![](assets/lstr-demo.gif)
*An interactive overview of a project's structure using `lstr`.*
## Philosophy

  - **Fast:** Runs directory scans in parallel by default to maximize speed on modern hardware.
  - **Minimalist:** Provides essential features without the bloat. The core experience is clean and uncluttered.
  - **Interactive:** An optional TUI mode for fluid, keyboard-driven exploration.

## Features

  - **High-performance:** Scans directories in parallel to be as fast as possible.
  - **Classic and interactive modes:** Use `lstr` for a classic `tree`-like view, or launch `lstr interactive` for a fully interactive TUI.
  - **Rich information display (optional):**
      - Display file-specific icons with `--icons` (requires a Nerd Font).
      - Show file permissions with `-p`.
      - Show file sizes with `-s`.
      - **Git Integration:** Show file statuses (`Modified`, `New`, `Untracked`, etc.) directly in the tree with the `-G` flag.
  - **Smart filtering:**
      - Respects your `.gitignore` files with the `-g` flag.
      - Control recursion depth (`-L`) or show only directories (`-d`).

## Installation

You need the Rust toolchain installed on your system to build `lstr`. Or use `nix develop` command to create development environment.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/bgreenwell/lstr.git
    cd lstr
    ```
2.  **Build and install using Cargo:**
    ```bash
    # This compiles in release mode and copies the binary to ~/.cargo/bin
    cargo install --path .
    ```

## Usage

```bash
lstr [OPTIONS] [PATH]
lstr interactive [OPTIONS] [PATH]
```

Note that `PATH` defaults to the current directory (`.`) if not specified.

| Option                 | Description                                                                 |
| :--------------------- | :-------------------------------------------------------------------------- |
| `-a`, `--all`          | List all files and directories, including hidden ones.                      |
| `--color <WHEN>`       | Specify when to use color output (`always`, `auto`, `never`).               |
| `-d`, `--dirs-only`    | List directories only, ignoring all files.                                  |
| `-g`, `--gitignore`    | Respect `.gitignore` and other standard ignore files.                       |
| `-G`, `--git-status`   | Show git status for files and directories.                                  |
| `--icons`              | Display file-specific icons; requires a [Nerd Font](https://www.nerdfonts.com/). |
| `-L`, `--level <LEVEL>`| Maximum depth to descend.                                                   |
| `-p`, `--permissions`  | Display file permissions (Unix-like systems only).                          |
| `-s`, `--size`         | Display the size of files.                                                  |
| `--expand-level <LEVEL>`| **Interactive mode only:** Initial depth to expand the interactive tree.   |

-----

## Interactive mode

Launch the TUI with `lstr interactive [OPTIONS] [PATH]`.

### Keyboard controls

| Key(s)  | Action                                                                                                                                      |
| :------ | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `k` | Move selection up. |
| `↓` / `j` | Move selection down. |
| `Enter` | **Context-aware action:**<br>- If on a file: Open it in the default editor (`$EDITOR`).<br>- If on a directory: Toggle expand/collapse. |
| `q` / `Esc` | Quit the application normally. |
| `Ctrl`+`s` | **Shell integration:** Quits and prints the selected path to stdout. |

## Examples

**1. List the contents of the current directory**

```bash
lstr
```

**2. Explore a project interactively, ignoring gitignored files**

```bash
lstr interactive -g --icons
```

**3. Display a directory with file sizes and permissions (classic view)**

```bash
lstr -sp
```

**4. See the git status of all files in a project**

```bash
lstr -aG
```

**5. Start an interactive session with all data displayed**

```bash
lstr interactive -gG --icons -s -p
```

## Piping and shell interaction

The classic `view` mode is designed to work well with other command-line tools via pipes (`|`).

### Interactive fuzzy finding with `fzf`

This is a powerful way to instantly find any file in a large project.

```bash
lstr -a -g --icons | fzf
```

`fzf` will take the tree from `lstr` and provide an interactive search prompt to filter it.

### Paging large trees with `less` or `bat`

If a directory is too large to fit on one screen, pipe the output to a *pager*.

```bash
# Using less (the -R flag preserves color)
lstr -L 10 | less -R

# Using bat (a modern pager that understands colors)
lstr --icons | bat
```

### Changing directories with `lstr`

You can use `lstr` as a visual `cd` command. Add the following function to your shell's startup file (e.g., `~/.bashrc`, `~/.zshrc`):

```bash
# A function to visually change directories with lstr
lcd() {
    # Run lstr and capture the selected path into a variable.
    # The TUI will draw on stderr, and the final path will be on stdout.
    local selected_dir
    selected_dir="$(lstr interactive -g --icons)"

    # If the user selected a path (and didn't just quit), `cd` into it.
    # Check if the selection is a directory.
    if [[ -n "$selected_dir" && -d "$selected_dir" ]]; then
        cd "$selected_dir"
    fi
}
```

After adding this and starting a new shell session (or running `source ~/.bashrc`), you can simply run:

```bash
lcd
```

This will launch the `lstr` interactive UI. Navigate to the directory you want, press `Ctrl+s`, and your shell's current directory will instantly change.

## Performance and concurrency

By default, `lstr` uses a parallel directory walker to maximize speed on multi-core systems. This parallelism is managed by the excellent [rayon](https://crates.io/crates/rayon) thread pool, which is used internally by `lstr`'s directory traversal engine.

For advanced use cases, such as benchmarking or limiting CPU usage, you can control the number of threads by setting the `RAYON_NUM_THREADS` environment variable before running the command.

**To force single-threaded (serial) execution:**

```bash
RAYON_NUM_THREADS=1 lstr .
```

## Inspiration

The philosophy and functionality of `lstr` are heavily inspired by the excellent C-based [tree](https://github.com/Old-Man-Programmer/tree) command line program. This project is an attempt to recreate that classic utility in modern, safe Rust.
