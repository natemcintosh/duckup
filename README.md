# Duckup

Just install the latest version of duckdb in my `~/.local/bin`. Designed for linux, since duckdb CLI can't yet be installed via a linux package manager.

Currently very dangerous: it just sticks it in `~/.local/bin/duckdb`, overwriting anything that might have been there. Use with great caution.

## Use
1. Have the [rust toolchain](https://www.rust-lang.org/tools/install) installed
1. Clone this repo
1. Run with `cargo run --release` to get the help message
1. Run with `cargo run --release -- update` to download and install the latest version of duckdb
