# lstr

[![Latest Version](https://img.shields.io/crates/v/lstr.svg)](https://crates.io/crates/lstr)
[![Changelog](https://img.shields.io/badge/Changelog-blue)](CHANGELOG.md)

A blazingly fast, minimalist directory tree viewer, written in Rust. Inspired by the command line program [tree](https://github.com/Old-Man-Programmer/tree).

![lstr screenshot](assets/screenshot.png)
*A clean overview of a project's structure, with specific Nerd Font icons for file types like Rust (``), Cargo configs (``), and licenses (``).*

## Philosophy

-   **Fast:** Runs directory scans in parallel by default to maximize speed on modern hardware.
-   **Minimalist:** Provides essential features without the bloat. The core experience is clean and uncluttered.
-   **Authentic:** Adheres to the spirit of classic command-line utilities.

## Features

-   Recursive directory listing with a visual tree structure.
-   Parallel directory traversal enabled by default for high performance.
-   Configurable colorized output for easy identification (`--color`).
-   Display file-specific icons via the `--icons` flag to quickly identify file types (requires a [Nerd Font](https://www.nerdfonts.com/)).
-   Control over listing depth (`-L`).
-   Option to display directories only (`-d`).
-   Support for showing hidden files (`-a`).
-   Ability to respect `.gitignore` and other standard ignore files via the `-g` flag.

## Installation

You need the Rust toolchain installed on your system to build **lstr**.

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/lstr.git
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
```

### **Arguments:**

-   `[PATH]`
    -   The directory path to list. Defaults to the current directory (`.`).

### **Options:**

-   `-g, --gitignore`
    -   Respect `.gitignore` and other standard ignore files.

-   `--color <WHEN>`
    -   Specify when to use color output (`always`, `auto`, `never`).

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

Here are a few common ways to use **lstr**.

**1. List the contents of the current directory**
```bash
lstr
```

**2. Display a directory two levels deep, ignoring gitignored files**
```bash
lstr -L 2 -g
```

**3. Show only the directory structure**
```bash
lstr -d
```

**4. Find all Rust files in a project**
```bash
lstr --color never | grep "\.rs$"
```

**8. Get a rich visual overview with icons**
Use the `--icons` flag for a modern, easy-to-parse view. This is best used with `-g` to hide build artifacts.
```bash
lstr -g --icons
```


## Performance & Concurrency

By default, **lstr** uses a parallel directory walker to maximize speed on multi-core systems. This parallelism is managed by the excellent [rayon](https://crates.io/crates/rayon) thread pool, which is used internally by **lstr**'s directory traversal engine.

For advanced use cases, such as benchmarking or limiting CPU usage, you can control the number of threads by setting the `RAYON_NUM_THREADS` environment variable before running the command.

**To force single-threaded (serial) execution:**
```bash
RAYON_NUM_THREADS=1 lstr .
```

**To limit the tool to 4 threads:**
```bash
RAYON_NUM_THREADS=4 lstr .
```

### Benchmarking

This project uses [hyperfine](https://github.com/sharkdp/hyperfine) for benchmarking performance:
```bash
hyperfine --warmup 1 --prepare 'sudo purge' --show-output 'lstr ~/Dropbox' 'lstr ~/Dropbox --icons' 'tree ~/Dropbox'
```
```
2307 directories, 14045 files
  Time (mean ± σ):     219.2 ms ±  44.5 ms    [User: 49.6 ms, System: 85.1 ms]
  Range (min … max):   161.2 ms … 287.5 ms    10 runs

Summary
  lstr ~/Dropbox ran
    1.37 ± 0.56 times faster than lstr ~/Dropbox --icons
    2.08 ± 0.49 times faster than tree ~/Dropbox
```


## Inspiration

The philosophy and functionality of **lstr** are heavily inspired by the excellent C-based [tree](https://github.com/Old-Man-Programmer/tree) command line program. This project is an attempt to recreate that classic utility in modern, safe Rust.