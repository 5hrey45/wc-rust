# Unix wc tool written in Rust

## Quick start
Git clone this repo, and run the cargo project with filename as argument.

```
cargo run text.txt
```

## Building the binary
Run cargo build --release to build the binary in the release mode.
cd into target/release

you will find the wc-rust (.exe in windows)

you can now use wc-rust filename or wc-rust -flag filename

```
wc-rust.exe ../../text.txt
wc-rust.exe -c ../../text.txt
wc-rust.exe -h ../../text.txt
wc-rust.exe -w ../../text.txt
```
