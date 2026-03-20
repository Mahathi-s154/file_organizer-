# file_organizer

`file_organizer` is a Rust CLI for safe, previewable file organization. The current behavior is extension-first, and the project is being shaped into a more capable file organization engine with stronger automation, reporting, and contributor workflows.

For example, a directory containing:

```text
downloads/
├── photo.png
├── report.txt
└── archive
```

becomes:

```text
downloads/
├── misc/
│   └── archive
├── png/
│   └── photo.png
└── txt/
    └── report.txt
```

## Why This Project

- Safe by default with dry-run support and collision-aware planning
- Useful both interactively and in scripts thanks to JSON reporting
- Cross-platform release automation with Cargo and npm packaging
- Small codebase with a roadmap toward richer planning and execution features

## Features

- Organizes files into extension-named folders such as `txt/`, `png/`, and `pdf/`
- Places files without an extension into `misc/`
- Supports dry runs with no filesystem changes
- Can scan recursively through subdirectories
- Skips hidden files and hidden directories
- Avoids overwriting by renaming collisions to `name (1).ext`, `name (2).ext`, and so on
- Leaves files alone when they are already in the correct target folder
- Emits structured run reports with `--format json`
- Falls back to copy-and-delete when a move crosses filesystems

## Requirements

- Rust and Cargo for building from source
- Node.js and npm for installing the packaged npm binary

## Installation

### From Cargo

Build and run locally:

```bash
cargo run -- --help
```

Install the binary from the current repository:

```bash
cargo install --path .
```

Then run:

```bash
file_organizer --help
```

### From npm

This project is also packaged for npm as `@mahathi154/file_organizer`.

Install it in an npm project:

```bash
npm install @mahathi154/file_organizer
```

Then run it with:

```bash
npx file_organizer --help
```

## Usage

```text
file_organizer [OPTIONS]
```

Options:

- `-t, --target-dir <TARGET_DIR>`: Directory to organize. Defaults to the current directory.
- `-d, --dry-run`: Print planned moves without moving any files.
- `-r, --recursive`: Include files from subdirectories.
- `--format <FORMAT>`: Output summary as `text` or `json`. Defaults to `text`.
- `-h, --help`: Show help.
- `-V, --version`: Show version.

## Examples

Organize the current directory:

```bash
file_organizer
```

Organize a specific directory:

```bash
file_organizer --target-dir ~/Downloads
```

Preview changes without moving files:

```bash
file_organizer --target-dir ~/Downloads --dry-run
```

Preview changes as JSON for another tool:

```bash
file_organizer --target-dir ~/Downloads --dry-run --format json
```

Organize a directory and all nested subdirectories:

```bash
file_organizer --target-dir ~/Downloads --recursive
```

Run without installing:

```bash
cargo run -- --target-dir ~/Downloads --dry-run
```

## How It Works

1. Scan the target directory tree.
2. Classify each file by its lowercase extension.
3. Use `misc` when a file has no extension.
4. Build a destination under `<target>/<extension>/filename`.
5. Add a numeric suffix when the destination already exists.
6. Execute the move plan or emit it as a dry run.
7. Report the outcome in text or JSON.

## Behavior Notes

- Non-recursive mode only processes files directly inside the target directory.
- Recursive mode moves matching files from nested subdirectories into root-level extension folders under the target directory.
- Hidden files and hidden directories are ignored.
- Files already inside the correct folder, such as `<target>/txt/file.txt`, are skipped.
- Cross-filesystem moves use a copy-and-delete fallback.
- Current hidden-file detection is based on dot-prefixed names.

## Architecture And Project Docs

- Design notes: [`docs/design.md`](docs/design.md)
- Contributor guide: [`CONTRIBUTING.md`](CONTRIBUTING.md)
- Roadmap: [`ROADMAP.md`](ROADMAP.md)

## Development

Run the full local quality bar:

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Show CLI help:

```bash
cargo run -- --help
```
