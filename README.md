<p align="center">
  <img src="assets/icon.png" width="128" height="128" alt="Shelf icon">
</p>

<h1 align="center">Shelf</h1>

<p align="center">
  <strong>Your markdown library, on your desktop.</strong>
</p>

<p align="center">
  A native macOS app that turns any folder of markdown files into a browsable, beautifully rendered library — with PDF export.
</p>

<p align="center">
  <a href="https://github.com/alteronauts/Shelf/releases"><img src="https://img.shields.io/github/v/release/alteronauts/Shelf?style=flat-square&color=blue" alt="Release"></a>
  <img src="https://img.shields.io/badge/platform-macOS%2011%2B-lightgrey?style=flat-square" alt="Platform">
  <img src="https://img.shields.io/badge/rust-2024%20edition-orange?style=flat-square" alt="Rust">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="License"></a>
</p>

---

## Features

- **Library browser** — Point Shelf at any folder. It organizes your `.md` files into shelves (folders) with horizontally scrollable book cards.
- **Markdown reader** — Full CommonMark rendering with syntax-highlighted code blocks, headings, lists, blockquotes, and more.
- **PDF export** — Convert any document to a cleanly typeset PDF with proper headings, bold/italic, code blocks, and bullet lists. Uses the [Inter](https://rsms.me/inter/) typeface.
- **Smart scanning** — Automatically skips `.git`, `node_modules`, `target`, `__pycache__`, and other noise. Fully customizable ignore patterns that persist between sessions.
- **Native & fast** — Built with Rust and [egui](https://github.com/emilk/egui). Universal binary for Apple Silicon and Intel.

## Install

### Download

Grab the latest `.dmg` from [**Releases**](https://github.com/alteronauts/Shelf/releases), open it, and drag **Shelf** to your Applications folder.

> [!NOTE]
> The app is ad-hoc signed. On first launch, right-click the app and choose **Open**, or run:
> ```bash
> xattr -cr /Applications/Shelf.app
> ```

### Build from source

Requires [Rust](https://rustup.rs/) (stable).

```bash
git clone https://github.com/alteronauts/Shelf.git
cd Shelf
cargo run            # debug build
```

To create a signed `.app` bundle and DMG installer:

```bash
./build-app.sh       # universal binary (arm64 + x86_64)
```

The script outputs `Shelf.app` and `Shelf-<version>.dmg` in the project root.

## How it works

```
Open a folder  →  Shelf scans for .md files  →  Browse shelves  →  Read  →  Export PDF
```

1. **Welcome screen** — Pick a folder containing your markdown files.
2. **Library** — Files are grouped by directory into horizontal shelves. Click any book card to open it.
3. **Reader** — Rendered markdown with a toolbar for navigating back or exporting to PDF.
4. **Ignore panel** — Toggle folders you want to hide. Your choices are saved automatically.

## Tech stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (2024 edition) |
| GUI | [eframe](https://github.com/emilk/egui) / [egui](https://github.com/emilk/egui) |
| Markdown | [egui_commonmark](https://crates.io/crates/egui_commonmark) + [pulldown-cmark](https://crates.io/crates/pulldown-cmark) |
| PDF | [genpdf](https://crates.io/crates/genpdf) |
| Typography | [Inter](https://rsms.me/inter/) (Regular, Bold, Italic, BoldItalic) |

## Project structure

```
src/
├── main.rs          # Entry point, window setup
├── app.rs           # App state machine, screen transitions
├── model.rs         # Book, Shelf, Library, Screen types
├── scanner.rs       # Folder scanning, ignore patterns
├── persist.rs       # Config persistence
├── export.rs        # PDF export with markdown parsing
└── ui/
    ├── welcome.rs   # Welcome screen
    ├── library.rs   # Library browser, ignore panel
    ├── shelf_row.rs # Horizontal book card row
    └── reader.rs    # Markdown reader + export button
```

## License

[MIT](LICENSE)
