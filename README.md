# file_organizer

`file_organizer` is a Rust CLI that sorts files into subdirectories based on file extension.

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

## Features

- Organizes files into extension-named folders such as `txt/`, `png/`, and `pdf/`
- Places files without an extension into `misc/`
- Supports dry runs with no filesystem changes
- Can scan recursively through subdirectories
- Skips hidden files and hidden directories
- Avoids overwriting by renaming collisions to `name (1).ext`, `name (2).ext`, and so on
- Leaves files alone when they are already in the correct target folder

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

Organize a directory and all nested subdirectories:

```bash
file_organizer --target-dir ~/Downloads --recursive
```

Run without installing:

```bash
cargo run -- --target-dir ~/Downloads --dry-run
```

## How It Works

1. Scans the target directory.
2. Determines each file's category from its lowercase extension.
3. Uses `misc` when a file has no extension.
4. Plans a destination under `<target>/<extension>/filename`.
5. Adds a numeric suffix when the destination name already exists.
6. Moves the files unless `--dry-run` is enabled.

## Behavior Notes

- Non-recursive mode only processes files directly inside the target directory.
- Recursive mode moves matching files from nested subdirectories into root-level extension folders under the target directory.
- Hidden files and hidden directories are ignored.
- Files already inside the correct folder, such as `<target>/txt/file.txt`, are skipped.
- The tool uses filesystem rename operations. Moves across different filesystems or mount points may fail.

## Development

Run tests:

```bash
cargo test
```

Show CLI help:

```bash
cargo run -- --help
```
