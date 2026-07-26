# jp-cli

A small Japanese dictionary and vocabulary manager for the terminal.

`jp` can look up Japanese words, save vocabulary to a local SQLite database,
and display saved words alongside an interactive dictionary search in a
terminal UI.

Dictionary results are provided by the [Jotoba](https://jotoba.de/) API.

## Features

- Look up words from the command line
- Show concise definitions or detailed readings
- Save and remove vocabulary locally
- Browse saved words in a terminal UI
- Keep data in a self-contained SQLite database

## Requirements

- Rust 1.85 or newer
- An internet connection for dictionary searches

## Installation

Clone the repository, then install the binary with Cargo:

```sh
git clone https://github.com/Samson026/jp-cli.git
cd jp-cli
cargo install --path .
```

This installs the command as `jp`.

To build without installing it:

```sh
cargo build --release
```

The compiled binary will be available at `target/release/jp`.

## Usage

### Look up a word

Searches can be made using romaji, kanji, or kana:

```sh
jp imi neko
jp imi 猫
jp imi ねこ
```

Use `--verbose` (or `-v`) to include kanji and kana readings for every result:

```sh
jp imi 猫 --verbose
```

Queries containing spaces should be quoted:

```sh
jp imi "to eat"
```

### Save a word

```sh
jp add 猫
```

The command looks up the word and stores the first definition returned by the
dictionary.

### List saved words

```sh
jp list
```

### Remove a saved word

```sh
jp remove 猫
```

### Open the terminal UI

```sh
jp tui
```

The terminal UI shows dictionary search on the left and saved vocabulary on
the right.

| Key | Action |
| --- | --- |
| Type | Enter a search query |
| `Backspace` | Delete the last character |
| `Enter` | Search |
| `Up` / `Down` | Scroll through the result |
| `q` | Quit |

## Data storage

Saved vocabulary is stored in `~/.jp-cli/jp.db`. On systems without a `HOME`
environment variable, `USERPROFILE` is used instead.

The database and its parent directory are created automatically the first time
you run `jp add`, `jp list`, `jp remove`, or `jp tui`.

## Development

Run the project directly through Cargo:

```sh
cargo run -- imi 日本 --verbose
```

Useful checks:

```sh
cargo fmt --check
cargo check
cargo test
```
