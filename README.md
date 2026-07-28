# groningen

`groningen` is a fast local-first machine translation CLI and TUI for Linux and macOS. It is designed as a portfolio-grade Rust systems project: modular, async, extension-oriented, and ready for a real inference backend such as ONNX, Candle, or CTranslate2.

## Highlights

- **Friendly dual-pane TUI** built with Ratatui and Crossterm.
- **Theme studio** with Catppuccin Mocha, Groningen Light, Nord Aurora, Dracula, Tokyo Night, Gruvbox Dark, Solarized Dark, Rosé Pine, Cyberpunk Neon, and Terminal Classic palettes.
- **In-app language installer** inspired by VS Code and LazyVim: focus the Languages pane and press `i` to install or select the highlighted language.
- **Pre-installed languages:** Dutch (`nl`, Nederlands), English (`en`, English), and Russian (`ru`, Русский).
- **Broad language catalog** where every language is shown with its English name and native name, for example German / Deutsch.
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
| `i` | Edit source text, install/select target language in Languages, or cycle theme in Settings |
| `Esc` | Return to normal mode |
| `Tab` | Switch Source → Target → Languages → Settings |
| `↑` / `↓` or `k` / `j` | Select a language in the in-app catalog |
| `s` | Set the highlighted installed language as the source language |
| `[` / `]` | Previous/next theme while Settings is focused |
| `Enter` | Translate the current source text |
| `q` | Quit |

The default pair is English → Dutch because Dutch, English, and Russian are available immediately. Other languages can be installed from inside the app by focusing the Languages pane and pressing `i`. Installed languages can be assigned as the source with `s`, while `i` selects the target.

## CLI usage

List bundled languages and installation status:

```bash
groningen list
```

Translate from a Unix pipe without opening the TUI:

```bash
echo "Computers process data" | groningen -s en -t nl
```

A terminal installer command is still available for automation, but the primary user flow is the in-app Languages pane:

```bash
groningen install de
```

## Language catalog

The bundled catalog includes pre-installed Dutch, English, and Russian plus installable entries for common ISO-639 language codes such as Afrikaans, Arabic, Azerbaijani, Belarusian, Bulgarian, Bengali, Bosnian, Catalan, Czech, Welsh, Danish, German / Deutsch, Greek, Spanish, Estonian, Persian, Finnish, French, Irish, Hebrew, Hindi, Croatian, Hungarian, Armenian, Indonesian, Icelandic, Italian, Japanese, Georgian, Kazakh, Korean, Lithuanian, Latvian, Macedonian, Malay, Norwegian, Polish, Portuguese, Romanian, Slovak, Slovenian, Albanian, Serbian, Swedish, Swahili, Thai, Turkish, Ukrainian, Urdu, Vietnamese, and Chinese.

## Architecture

```text
src/
├── main.rs         # Event loop, MPSC channel router, CLI entry point
├── cli.rs          # Clap definitions and shell piping mode
├── app.rs          # AppState, input modes, themes, language selection, mutations
├── tui/
│   ├── mod.rs      # Raw terminal lifecycle management
│   ├── ui.rs       # Ratatui layouts, theme studio, panes, status bar
│   └── events.rs   # Crossterm event listener to MPSC
├── manager/
│   ├── mod.rs      # Model downloader and config/model directory setup
│   └── registry.rs # Language manifest parser and bundled catalog
└── engine/
    ├── mod.rs      # TranslationEngine trait
    └── dummy.rs    # Mock offline inference engine
```

Model files are stored under your platform config directory, typically:

```text
~/.config/groningen/models/
```

## Development status

This repository currently ships a mock engine and mock language URLs so the UI/CLI flow can be reviewed without large model artifacts. Failed network downloads create a deterministic mock `.bin` language file, which keeps the install flow demo-friendly while preserving the async downloader path for future real registries.
