# ctee

> Rust implementation of Unix's `tee` with stripping of ANSI colors.

## Installation

Use the crate published on [crates.io](https://crates.io/crates/ctee):
```shell
cargo install ctee
```

OR manually clone and install the latest version:
```shell
git clone https://github.com/Lipen/ctee
cd ctee
cargo install --path .
```

## Usage

```
$ ctee --help

Read from STDIN and write to STDOUT (unchanged) and FILES (without ANSI-colors)

USAGE:
    ctee [OPTIONS] [FILES]...

ARGS:
    <FILES>...    Output files

OPTIONS:
    -a, --append                     Append to the given files instead of overwriting
        --bs <BUFFER_SIZE>           Buffer size [default: 8192]
    -h, --help                       Print help information
        --strip-ansi <STRIP_ANSI>    Strip ANSI color codes when writing to files [default: true]
    -V, --version                    Print version information

EXAMPLES:
    <command-with-colorized-output> | ctee my.log
```

## CLI

