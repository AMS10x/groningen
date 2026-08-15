# Groningen

Groningen is a local-first machine translation CLI and terminal UI written in Rust. It demonstrates a polished product flow for installing language-pair "extensions", translating from either an interactive workspace or Unix pipes, and swapping a mock engine for a real local inference backend later.

> Status: demo-ready prototype. The bundled model URLs intentionally point at mock locations; if a download fails, Groningen writes a deterministic mock model file so the install and activation flow can still be reviewed offline.

## Why this project exists

Groningen is built as a portfolio-grade Rust systems app with clear seams for future work:

- a `TranslationEngine` trait for real ONNX, Candle, or CTranslate2 adapters;
- async model download/install plumbing;
- a responsive Ratatui UI that does not block while translating or downloading;
- a small CLI that works well in shell scripts.

## Features

- **Interactive dual-pane TUI** for source and translated text.
- **Language extension drawer** with one-key-per-action controls and install/activate behavior inspired by VS Code and LazyVim.
- **Installed-model detection** when the TUI starts, so previously downloaded extensions are marked immediately.
- **Active language-pair routing** with Spanish, Italian, German, Russian, and French preinstalled for immediate use.
- **Theme switcher** with Catppuccin Mocha, Groningen Light, and Terminal Classic.
- **Unix pipe mode** for scriptable translation.
- **Offline-friendly mock engine** that translates a small vocabulary and reverses unknown words.

## Requirements

- Rust stable toolchain, edition 2021 compatible.
- Linux or macOS terminal with ANSI color support.

## Installation

```bash
git clone https://github.com/AMS10x/groningen.git
cd groningen
cargo build --release
```

The binary is created at:

```text
target/release/groningen
```

Optional local install:

```bash
cargo install --path .
```

## Quick start

Open the TUI:

```bash
groningen list
```

Translate from a pipe:

```bash
echo "hello world" | cargo run -- -t it
```

Activate or install a bundled language-pair extension:

```bash
cargo run -- install en-it
```

List bundled preinstalled extensions and install status:

```bash
cargo run -- list
```

## TUI controls

| Key | Action |
| --- | --- |
| `e` | Edit source text |
| `x` | Activate the selected preinstalled extension, or install it if a model file is missing |
| `t` | Cycle theme |
| `c` | Clear source and translation buffers |
| `Esc` | Return to normal mode |
| `Tab` | Switch Source → Target → Extensions → Settings |
| `↑` / `↓` or `k` / `j` | Select a language extension |
| `Enter` | Translate the current source text with the active language pair |
| `q` or `Ctrl+C` | Quit |

## CLI reference

```text
groningen [OPTIONS] [COMMAND]
```

Options:

| Option | Description | Default |
| --- | --- | --- |
| `-s, --source <SOURCE_LANG>` | Source language code for pipe mode | `en` |
| `-t, --to <TARGET_LANG>` | Target language code for pipe mode | `it` |
| `-h, --help` | Show help | |
| `-V, --version` | Show version | |

Commands:

| Command | Description |
| --- | --- |
| `install <PAIR>` | Install a language-pair model, for example `en-it` |
| `list` | Show available extensions and whether each model file is installed |

Pipe-mode examples:

```bash
echo "computers process data" | groningen -s en -t it
printf 'fast local translation\n' | groningen --source en --to de
```

## Project layout

```text
src/
├── main.rs         # Tokio runtime, TUI event loop, task spawning
├── cli.rs          # Clap commands, pipe mode, extension list/install output
├── app.rs          # AppState, input modes, panes, themes, state transitions
├── tui/
│   ├── mod.rs      # Raw terminal lifecycle management
│   ├── ui.rs       # Ratatui layouts, colors, panes, status bar
│   └── events.rs   # Crossterm event listener to MPSC
├── manager/
│   ├── mod.rs      # Config/model directories and downloader
│   └── registry.rs # Extension manifests and bundled registry
└── engine/
    ├── mod.rs      # TranslationEngine trait
    └── dummy.rs    # Mock local inference engine
```

Model files are stored under your platform config directory, typically:

```text
~/.config/groningen/models/
```

## Development

Format and check the project:

```bash
cargo fmt
cargo check
```

Run the app locally:

```bash
cargo run
```

## Roadmap

- Wire installer URLs to manifest data instead of the current mock URL builder.
- Add a remote registry command once installer behavior is backed by manifest data.
- Add checksum verification for model files.
- Add a real inference backend behind `TranslationEngine`.
- Persist user settings such as theme and last active language pair.
- Add integration tests for CLI output and registry parsing.

## License

No license file is currently included. Add one before publishing packages or accepting external contributions.
