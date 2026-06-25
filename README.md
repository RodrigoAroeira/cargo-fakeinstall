# cargo-fakeinstall

Install a "fake" binary that downloads and replaces itself with the real binary
on first run.

Useful for installing CLI tools that aren't available on crates.io via the
familiar `cargo install` workflow. Instead of compiling from source, the installed
bootstrapper fetches the real binary from a URL (e.g. a GitHub release asset) at launch
time.

## Installation

```bash
git clone https://github.com/RodrigoAroeira/cargo-fakeinstall
cargo install --path cargo-fakeinstall
```

## Usage

```bash
cargo fakeinstall --uri <URI> --bin-name <NAME>

# Shorthand:
cargo fakeinstall -u <URI> -b <NAME>
```

The `--uri` argument accepts:
- A **remote URL** (e.g. a GitHub release asset)
- A **local file** path prefixed with `file://` (e.g. `file:///path/to/binary`)

### Examples

Install `jq` (a JSON processor, not on crates.io) from a remote URL:

```bash
cargo fakeinstall \
  -u https://github.com/jqlang/jq/releases/download/jq-1.8.2/jq-linux-amd64 \
  -b jq
```

Install a local pre-downloaded binary:

```bash
cargo fakeinstall -u file://./downloaded-binary -b mytool
```

First run downloads/copies the real binary and replaces the bootstrapper:

```bash
$ jq --version
# (downloads real jq, replaces itself, exits)
```

Subsequent runs use the real binary:

```bash
$ jq --version
jq-1.8.2
```

## Why?

I wanted a quick way to install a pre-built binary somewhere already in `PATH`
without manually downloading files or messing with `chmod` or using package managers.

## How it works

1. A temporary Cargo project is created with a `main.rs` that contains the bootstrapper.
2. `cargo install --path` builds and installs the bootstrapper user-wide.
3. When the user runs the binary for the first time, the bootstrapper:
   - For remote URIs: downloads the real binary with `wget`
   - For local URIs: copies the file from the given path
   - Marks it executable (`chmod 755`)
   - Replaces itself (the bootstrapper) with the real binary

Subsequent runs execute the real binary directly.

The download can't happen during `cargo install` because Cargo needs to compile
a Rust crate to register the binary (e.g. in `cargo install --list`). The bootstrapper
is that crate: a binary that Cargo registers, which then
replaces itself with the real binary on first run.

## Requirements

- Linux (uses `std::os::unix::fs::PermissionsExt`).
- `wget` must be installed for remote URIs (not needed for `file://` URIs).

## License

MIT
