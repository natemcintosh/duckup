# Duckup

Just install the latest version of duckdb in my `~/.local/bin`. Designed for linux, since duckdb CLI can't yet be installed via a linux package manager.

Currently very dangerous: it just sticks it in `~/.local/bin/duckdb`, overwriting anything that might have been there. Use with great caution.

## Use
1. Have the [rust toolchain](https://www.rust-lang.org/tools/install) installed
1. Run `cargo install duckup`
1. Run `duckup` for help message
1. Run `duckup update` to install the latest version of the duckdb CLI into `~/.local/bin`
