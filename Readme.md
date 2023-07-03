# Rust-FSTest

A Rust-based simple fs benchmark, refer to https://github.com/LLNL/mdtest.git

## Compile 

```
cargo build --release
```

## Run

Usage: rustfstest [OPTIONS]

```
./target/release/rustfstest --help
Options:
  -r, --rounds <ROUNDS>
          number of rounds the test will run [default: 10]
  -b, --bytes <BYTES>
          bytes to read or write [default: 4096]
  -v, --verbose
          verbosity value
  -h, --help
          Print help
  -V, --version
          Print version
```

