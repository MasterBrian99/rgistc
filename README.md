# rgistc

### command line utility to create secret gist.

#### Usage

```
Simple to program to create github Gists

Usage: rgistc [OPTIONS] --file <FILE> --key <KEY>

Options:
  -f, --file                       Name and content for the file that make up the gist
  -k, --key <KEY>                  Github Authorization Key
  -d, --description <DESCRIPTION>  [default: ]
  -h, --help                       Print help information
  -V, --version                    Print version information

```

```
rgistc --file README.md --key=ghp_.........
```

#### Build From Source

- Clone the Repository

```
https://github.com/MasterBrian99/rgistc.git
```

- Run Cargo Build

```
cargo build
```
