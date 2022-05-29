# Building from source

## Dependencies

To build from source, you need:

- A Rust compiler
- A C compiler
- `binutils`
- `make`
- `perl`

Install Rust with [`rustup`](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install everything else:

```bash
# This command works for Debian/Ubuntu
# For other Linux distros, install these packages with the system's package manager
sudo apt-get install build-essential binutils make perl
```

## Compile

Clone the repo:

```bash
git clone https://github.com/chrisx8/contact-rs.git
cd contact-rs
```

Compile release (optimized) binary:

```bash
# `nproc` doesn't exist on MacOS. Replace with the number of cores on your machine.
cargo build --jobs $(nproc) --locked --release
# Strip debug symbols
strip target/release/contact-rs
# The binary is at `target/release/contact-rs`
```

If you need the debug (unoptimized + debug symbols) binary for development:

```bash
cargo build --jobs $(nproc) --locked
# The binary is at `target/debug/contact-rs`
```
