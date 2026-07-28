# groningen

`groningen` is a fast local-first machine translation CLI and TUI for Linux and macOS. It is designed as a portfolio-grade Rust systems project: modular, async, extension-oriented, and ready for a real inference backend such as ONNX, Candle, or CTranslate2.

## Highlights

- **Friendly dual-pane TUI** built with Ratatui and Crossterm.
- **Themeable settings pane** with Catppuccin Mocha, Groningen Light, and Terminal Classic palettes.
- **Extension-style language installer** inspired by VS Code and LazyVim: focus the Extensions pane and press `i` to install the selected language pair.
- **Responsive event loop** using Tokio MPSC channels so downloads and translation never block rendering.
- **Unix pipe mode** for scripts and shell workflows.
- **Pluggable engine trait** with a dummy offline engine for quick demos.

## Install

```bash
git clone https://github.com/AMS10x/groningen.git
cd groningen
cargo build --release
```

The binary will be available at `target/release/groningen`.

## TUI usage

```bash
cargo run
```

Keybindings:

| Key | Action |
| --- | --- |
| `i` | Edit source text, install selected extension, or cycle theme depending on the focused pane |
| `Esc` | Return to normal mode |
| `Tab` | Switch Source → Target → Extensions → Settings |
| `↑` / `↓` or `k` / `j` | Select a language extension |
| `Enter` | Translate the current source text |
| `Ctrl+D` | Download/install the active language model |
| `q` | Quit |

## CLI usage

Install a language extension:

```bash
groningen install en-it
```

List bundled extensions:

```bash
groningen list
```

Translate from a Unix pipe without opening the TUI:

```bash
echo "Computers process data" | groningen -t it
```

## Architecture

```text
src/
├── main.rs         # Event loop, MPSC channel router, CLI entry point
├── cli.rs          # Clap definitions and shell piping mode
├── app.rs          # AppState, input modes, themes, extension selection, mutations
├── tui/
│   ├── mod.rs      # Raw terminal lifecycle management
│   ├── ui.rs       # Ratatui layouts, theme styling, panes, status bar
│   └── events.rs   # Crossterm event listener to MPSC
├── manager/
│   ├── mod.rs      # Model downloader and config/model directory setup
│   └── registry.rs # Extension manifest parser and bundled index
└── engine/
    ├── mod.rs      # TranslationEngine trait
    └── dummy.rs    # Mock offline inference engine
```

Model files are stored under your platform config directory, typically:

```text
~/.config/groningen/models/
```

## Development status

This repository currently ships a mock engine and mock extension URLs so the UI/CLI flow can be reviewed without large model artifacts. Failed network downloads create a deterministic mock `.bin` model file, which keeps the install flow demo-friendly while preserving the async downloader path for future real registries.
