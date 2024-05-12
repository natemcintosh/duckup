# Duckup

Since duckdb CLI can't yet be installed via a linux package manager, this will install the latest version of the duckdb CLI on your computer. Default location is `~/.local/bin/`, but you can choose your own location with the `--folder_path` option.

Use with caution, it will overwrite an existing binary.

## Goals

It would be nice to eventually make this tool work like the wonderful [`rustup`](https://github.com/rust-lang/rustup) and [`juliaup`](https://github.com/JuliaLang/juliaup) tools. However, there is a very long way to go to reach that.

## Use
1. Have the [rust toolchain](https://www.rust-lang.org/tools/install) installed
1. Run `cargo install duckup`
1. Run `duckup` for help message
1. Run `duckup update` to install the latest version of the duckdb CLI into `~/.local/bin`. Run `duckup update --folder_path /path/to/desired/location/` to install to a location of your choice.
