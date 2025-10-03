<p align="center">
    <img src="./.github/images/logo-dark.png#gh-dark-mode-only" width="300">
    <img src="./.github/images/logo-light.png#gh-light-mode-only" width="300">
    <br>
    <br>
    <em>a tiny workbench for parsing, visualizing & analyzing linker artifacts.</em>
    <br>
    <br>
</p>

🗺️🔍 Analyze `.map` files interactively with a TUI, export metrics to JSON/CSV for CI pipelines, and quickly identify bloated sections and symbols.

> [!IMPORTANT]  
> **Platform Support**: Currently supports **macOS (Mach-O)** linker map files only. Linux (ELF) support is coming soon.

![Demo](./.github/demo.gif)

## Quickstart

Install `linkerland` with `cargo`:

```bash
cargo install linkerland
```

After the installation, you're all set! 💯

Dive into your linker map:

```bash
linkerland path/to/app.map

# or explicitly:
linkerland viz path/to/app.map
```

## Features

### Interactive TUI (viz)

Explore your linker map file with a terminal user interface featuring:

- **Object table**: Browse all object files with their TEXT, DATA, BSS, and TOTAL sizes.
- **Symbol table**: Drill down into symbols for the selected object, categorized by bucket (TEXT/DATA/BSS/OTHER).
- **Filtering**: Press `/` to search/filter objects or symbols by name.
- **Sorting**: Press `s` to cycle through sort keys (Total, Text, Data, Bss, Path for objects; Size, Address, Name for symbols).
- **Units toggle**: Press `u` to switch between human-readable (KiB, MiB) and hex (0x...) formats.
- **Navigation**: Arrow keys to move, `Tab` to switch panes, `r` to reverse sort order.

> See [Keybindings](#keybindings) for full reference.

### Export Metrics

Export your linker map analysis to structured formats for scripting, CI, or further processing:

```bash
linkerland export path/to/app.map --format json --out analysis.json
linkerland export path/to/app.map --format csv --out analysis.csv
```

Ideal for tracking binary size growth over time, alerting on regressions, or integrating into build dashboards.

## Installation

### From crates.io

```bash
cargo install linkerland
```

### From source

```bash
git clone https://github.com/shrirambalaji/linkerland.git
cd linkerland
cargo install --path .
```

## Usage

### Commands

```bash
# Launch interactive TUI (default if .map provided)
linkerland <path-to-map>
linkerland viz <path-to-map>

# Export to JSON
linkerland export <path-to-map> --format json --out output.json

# Export to CSV
linkerland export <path-to-map> --format csv --out output.csv
```

### Keybindings

| Key       | Action                                                                     |
| --------- | -------------------------------------------------------------------------- |
| `↑` / `↓` | Navigate up/down in active pane                                            |
| `Tab`     | Switch between Objects and Symbols panes                                   |
| `/`       | Start filter (type to filter, Backspace to edit)                           |
| `s`       | Cycle sort key (Total → Text → Data → Bss → Path or Size → Address → Name) |
| `r`       | Reverse sort order                                                         |
| `u`       | Toggle display units (human ↔ hex)                                         |
| `?`       | Show help overlay                                                          |
| `q`       | Quit                                                                       |
| `Esc`     | Close help overlay                                                         |

## Architecture

**linkerland** has the following crates:

- **parser**: Winnow-based parser for `.map` files; handles sections, symbols, addresses, sizes.
- **metrics**: Aggregates parsed data into per-object and per-symbol metrics; classifies sections into buckets (TEXT/DATA/BSS/OTHER).
- **cli**: Clap-based CLI with `viz` and `export` subcommands.
- **tui**: Ratatui-based interactive terminal interface with filtering, sorting, scrolling, and unit toggling.

## License

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

## Copyright

Copyright © 2025, [Shriram Balaji](https://github.com/shrirambalaji)
