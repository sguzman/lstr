# lstr

[![Latest Version](https://img.shields.io/crates/v/lstr.svg)](https://crates.io/crates/lstr)
[![Changelog](https://img.shields.io/badge/changelog-v0.1.0-blue)](CHANGELOG.md)


A blazingly fast, minimalist directory tree viewer, written in Rust.

`lstr` (`LiSt-TRree`) walks a directory and prints its contents in a tree structure. It is designed with the Unix philosophy in mind: do one thing and do it well, with a focus on speed and simplicity.

![**Fig.** Screenshot of output from **lstr**.](assets/screenshot.png) 

## Philosophy

-   **Fast:** Written in Rust for maximum performance and memory efficiency.
-   **Minimalist:** Provides essential features without the bloat. The core experience is clean and uncluttered.
-   **Authentic:** Adheres to the spirit of classic command-line utilities.

## Features

-   Recursive directory listing with a visual tree structure.
-   Parallel directory traversal enabled by default for high performance.
-   Configurable colorized output for easy identification (`--color`).
-   Control listing depth (`-L`).
-   Option to list directories only (`-d`).
-   Support for showing hidden files (`-a`).

### Color Output

Color is enabled by default for interactive terminals. To disable color, set the `NO_COLOR` environment variable.

```bash
# This command will have no colored output
NO_COLOR=1 lstr
```

## Installation

You need the Rust toolchain installed on your system to build `lstr`.

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/your-username/lstr.git](https://github.com/your-username/lstr.git)
    cd lstr
    ```

2.  **Build and install using Cargo:**
    ```bash
    # This compiles in release mode and copies the binary to ~/.cargo/bin
    cargo install --path .
    ```

## Usage

```
lstr [OPTIONS] [PATH]
```

### **Arguments:**

-   `[PATH]`
    -   The directory path to list. Defaults to the current directory (`.`).

### **Options:**

-   `--serial`
    -   Run in single-threaded (serial) mode. Parallelism is enabled by default to maximize speed.

-   `--color <WHEN>`
    -   Specify when to use color output.
    -   `always`: Always use color, even when piping to a file.
    -   `auto`: Use color only when printing to a terminal (default).
    -   `never`: Never use color.

-   `-L, --level <LEVEL>`
    -   Maximum depth to descend.

-   `-d, --dirs-only`
    -   List directories only, ignoring all files.

-   `-a, --all`
    -   List all files and directories, including hidden ones.

-   `-h, --help`
    -   Show the help message.

-   `-V, --version`
    -   Show the version information.

## Examples

Here are a few common ways to use `lstr`.

**1. List the contents of the current directory**
This is the default behavior. `lstr` will run in parallel with auto-detected color.

```bash
lstr
```

**2. Display a directory two levels deep**
Use the `-L` flag to control recursion depth. This is useful for getting a quick overview without too much detail.

```bash
lstr -L 2 ~/Documents
```

**3. Show only the directory structure**
Hide all files and focus on the layout of your directories.

```bash
lstr -d
```

**4. Find all Markdown files in a project**
Disable color for clean output that can be piped to other tools like `grep`.

```bash
lstr --color never | grep "\.md$"
```

**5. See everything, including hidden files**
The `-a` flag will show dotfiles like `.git`, `.gitignore`, and `.vscode`.

```bash
lstr -a
```

**6. Run in single-threaded (serial) mode**
This fulfills the user's request. This disables the default parallel behavior, which can be useful for debugging, consistent benchmarking, or on systems with very few cores.

```bash
lstr --serial
```

**7. Combine flags for a power-user view**
Show all files (`-a`) up to a depth of 2 (`-L 2`) in your project, forcing color on.

```bash
lstr -a -L 2
```

## Future Improvements

`lstr` is a living project. Future enhancements could include:
-   Gitignore awareness (`-g` flag).
-   File size display (`-s` flag).
-   Permissions and metadata display.
-   Optimized parallel directory traversal for massive directories.

## Benchmarking

This project uses [hyperfine](https://github.com/sharkdp/hyperfine) for benchmarking:
```bash
hyperfine --warmup 1 --prepare 'sudo purge' --show-output 'lstr ~/Dropbox' 'lstr --serial ~/Dropbox' 'tree ~/Dropbox'
```
```
2288 directories, 13906 files
  Time (mean ± σ):     224.6 ms ±  55.0 ms    [User: 48.6 ms, System: 88.2 ms]
  Range (min … max):   162.8 ms … 316.0 ms    10 runs

Summary
  lstr ~/Dropbox ran
    1.26 ± 0.46 times faster than lstr --serial ~/Dropbox
    1.45 ± 0.46 times faster than tree ~/Dropbox
```

## Inspiration

The philosophy and functionality of `lstr` are heavily inspired by the excellent C-based [tree](https://github.com/Old-Man-Programmer/tree) project. This project is an attempt to recreate that classic utility in modern, safe Rust. (It's also an excuse to learn rust!)
