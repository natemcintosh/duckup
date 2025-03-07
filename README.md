# Duckup

## Archive Status
With the ability to install duckdb directly on Unix systems via an [install script](https://duckdb.org/2025/03/06/gems-of-duckdb-1-2.html#installation-script-on-linux-and-macos), this tool has reached the end of its useful life. If you have been using `duckup` to manage your linux installations, please note that using DuckDB's install script will install the CLI to `~/.duckdb/cli`, whereas this tool installed the CLI to `~/.local/bin/`. Please adjust your `$PATH` accordingly.

### Remvoing duckup, and the DuckDB CLI binary it installed
To remove the binary this tool installed (assuming you used the default install location), run `rm ~/.local/bin/duckdb`. Then to remove `duckup` from your machine, run `cargo uninstall duckup`.

## About
Since duckdb CLI can't yet be installed via a linux package manager, this will install the latest version of the duckdb CLI on your computer. Default location is `~/.local/bin/`, but you can choose your own location with the `--folder_path` option.

Use with caution, it will overwrite an existing binary.

## Goals

It would be nice to eventually make this tool work like the wonderful [`rustup`](https://github.com/rust-lang/rustup) and [`juliaup`](https://github.com/JuliaLang/juliaup) tools. However, there is a very long way to go to reach that.

## Use
1. Have the [rust toolchain](https://www.rust-lang.org/tools/install) installed
1. Run `cargo install duckup`
1. Run `duckup` for help message
1. Run `duckup update` to install the latest version of the duckdb CLI into `~/.local/bin`. Run `duckup update --folder_path /path/to/desired/location/` to install to a location of your choice.
