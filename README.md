# lstr

[![Latest Version](https://img.shields.io/crates/v/lstr.svg)](https://crates.io/crates/lstr)
[![Changelog](https://img.shields.io/badge/Changelog-blue)](CHANGELOG.md)

A blazingly fast, minimalist directory tree viewer, written in Rust. Inspired by the command line program [tree](https://github.com/Old-Man-Programmer/tree), with a powerful interactive mode.

![](assets/lstr-demo.gif)
*An interactive overview of **lstr**'s project structure... using **lstr**.*

## Philosophy

  - **Fast:** Runs directory scans in parallel by default to maximize speed on modern hardware.
  - **Minimalist:** Provides essential features without the bloat. The core experience is clean and uncluttered.
  - **Interactive:** An optional TUI mode for fluid, keyboard-driven exploration.

## Features

### Classic view

  - Recursive directory listing with a visual tree structure.
  - Parallel directory traversal enabled by default for high performance.
  - Configurable colorized output for easy identification (`--color`).
  - Control over listing depth (`-L`).
  - Option to display directories only (`-d`).

### Interactive mode (`lstr interactive`)

  - A terminal-based user interface for navigating the file tree.
  - Expand and collapse directories on the fly.
  - Open any file in your default editor (`$EDITOR`) with the `Enter` key.
  - Integrates with your shell for quickly changing directories.
  - Supports `--icons`, `--gitignore`, and `--all` flags.
  - Set the initial expansion depth with `--expand-level`.

## Installation

You need the Rust toolchain installed on your system to build **lstr**.

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

    Once published, it can be installed with `cargo install lstr`.

## Usage

```
lstr [OPTIONS] [PATH]
lstr interactive [OPTIONS] [PATH]
```

### **Options (classic view):**

  - `-L, --level <LEVEL>`: Maximum depth to descend.
  - `-d, --dirs-only`: List directories only, ignoring all files.
  - `--color <WHEN>`: Specify when to use color output (`always`, `auto`, `never`).

### **Options (both views):**

  - `-g, --gitignore`: Respect `.gitignore` and other standard ignore files.
  - `-a, --all`: List all files and directories, including hidden ones.
  - `--icons`: Display file-specific icons (requires a [Nerd Font](https://www.nerdfonts.com/)).

### **Options (interactive mode):**

  - `--expand-level <LEVEL>`: Initial depth to expand the directory tree.

-----

## Interactive mode

Launch the TUI with `lstr interactive`.

### Keyboard controls

| Key(s) | Action |
| :--- | :--- |
| `↑` / `k` | Move selection up. |
| `↓` / `j` | Move selection down. |
| `Enter` | **Context-aware action:**\<br/\>- If on a file: Open it in the default editor (`$EDITOR`).\<br/\>- If on a directory: Toggle expand/collapse. |
| `q` / `Esc` | Quit the application normally. |
| `Ctrl+s` | **Select and quit**: Quit and print the selected path to standard output for `cd` integration. |

## Examples

**1. List the contents of the current directory**

```bash
lstr
```

**2. Explore a project interactively, ignoring gitignored files**

```bash
lstr interactive -g --icons
```

**3. Start an interactive session expanded two levels deep**

```bash
lstr interactive --expand-level 2
```

**4. Display a directory two levels deep (classic view)**

```bash
lstr -L 2 -g
```

## Piping and Shell Interaction

The classic `view` mode is designed to work well with other command-line tools via pipes (`|`).

### Interactive Fuzzy Finding with **fzf**

This is a powerful way to instantly find any file in a large project.

```bash
lstr -a -g --icons | fzf
```

**fzf** will take the tree from **lstr** and provide an interactive search prompt to filter it.

### Paging Large Trees with less or bat

If a directory is too large to fit on one screen, pipe the output to a *pager*.

```bash
# Using less (the -R flag preserves color)
lstr -L 10 | less -R

# Using bat (a modern pager that understands colors)
lstr --icons | bat
```

## Performance and concurrency

By default, **lstr** uses a parallel directory walker to maximize speed on multi-core systems. This parallelism is managed by the excellent [rayon](https://crates.io/crates/rayon) thread pool, which is used internally by **lstr**'s directory traversal engine.

For advanced use cases, such as benchmarking or limiting CPU usage, you can control the number of threads by setting the `RAYON_NUM_THREADS` environment variable before running the command.

**To force single-threaded (serial) execution:**

```bash
RAYON_NUM_THREADS=1 lstr .
```

## Inspiration

The philosophy and functionality of **lstr** are heavily inspired by the excellent C-based [tree](https://github.com/Old-Man-Programmer/tree) command line program. This project is an attempt to recreate that classic utility in modern, safe Rust.