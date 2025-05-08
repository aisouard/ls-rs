# ls-rs

A simple directory listing utility written in Rust.

## Description

`ls-rs` is a minimal implementation of the Unix `ls` command that displays the names of files and directories in a specified path. If no path is provided, it lists the contents of the current directory.

## Installation

Clone this repository and build the project using Cargo:

```bash
git clone https://github.com/username/ls-rs.git
cd ls-rs
cargo build --release
```

The compiled binary will be available at `target/release/ls-rs`.

## Usage

```bash
# List contents of current directory
ls-rs

# List contents of a specific directory
ls-rs /path/to/directory
```

## Features

- List files and directories in a given path
- Simple error handling for invalid paths
- Zero dependencies

## License

This project is open source and available under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.