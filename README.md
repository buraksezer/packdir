# packdir

A CLI tool to compress and decompress directories using zstd compression.

## Installation

```bash
cargo build --release
```

The binary will be available at `./target/release/packdir`.

## Usage

### Compress a directory

```bash
packdir compress <NAME> <DIRECTORY> <DESTINATION>
```

**Arguments:**
- `NAME` - Name of the resulting file (without extension)
- `DIRECTORY` - Path to the directory to compress
- `DESTINATION` - Destination directory for the compressed file

**Example:**
```bash
packdir compress mybackup ./my-folder /tmp/backups
# Creates: /tmp/backups/mybackup-1766865050.zstd
```

### Decompress an archive

```bash
packdir decompress <FILE> <DESTINATION>
```

**Arguments:**
- `FILE` - Path to the compressed file
- `DESTINATION` - Destination directory for extraction

**Example:**
```bash
packdir decompress /tmp/backups/mybackup-1766865050.zstd /tmp/restored
# Extracts to: /tmp/restored/my-folder/
```

## Features

- Uses zstd for fast, efficient compression
- Preserves directory structure
- Preserves file permissions
- Timestamps output files to avoid overwrites

## Dependencies

- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [zstd](https://crates.io/crates/zstd) - Zstandard compression
- [tar](https://crates.io/crates/tar) - Tar archive creation and extraction

## License

MIT
