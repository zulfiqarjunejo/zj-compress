# zj-compress

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

This is a simple compression tool written in Rust.

## compilation

Execute the following command to create a production-grade binary.

```bash
cargo build --release
```

## usage

Execute the following command(s) to compress `source` to `target`

```bash
cp ./target/release/zj-compress ./
./zj-compress README.md README.compressed
```