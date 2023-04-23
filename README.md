# Chickadee-rs

A rust implementation of [chickadee](https://github.com/chapinb/chickadee)

[![CI|cargo checkmate](https://github.com/chapinb/chickadee-rs/actions/workflows/cargo-checkmate.yaml/badge.svg?branch=main)](https://github.com/chapinb/chickadee-rs/actions/workflows/cargo-checkmate.yaml)

## ⚠️  WORK IN PROGRESS ⚠️

This is under development. Please do not use.

## Development

### Running profiling

1. Install perf
2. Install flamegraph, `cargo install flamegraph`
3. Build the binary in debug mode, `cargo build`
4. Run the tool with the args, `flamegraph -- target/debug/chickadee --ips test_data/test.ips.txt --columns query,lat,lon`
