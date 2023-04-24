# Chickadee-rs

A rust implementation of [chickadee](https://github.com/chapinb/chickadee)

[![CI|cargo checkmate](https://github.com/chapinb/chickadee-rs/actions/workflows/cargo-checkmate.yaml/badge.svg?branch=main)](https://github.com/chapinb/chickadee-rs/actions/workflows/cargo-checkmate.yaml)
[![CI|cli run](https://github.com/chapinb/chickadee-rs/actions/workflows/cargo-run.yaml/badge.svg?branch=main)](https://github.com/chapinb/chickadee-rs/actions/workflows/cargo-run.yaml)

## ⚠️  WORK IN PROGRESS ⚠️

This is under development. Please do not use.

## Usage

This is a command line application that extracts a collection of IP addresses from
a source and enriches them with the ip-api.com API.

More APIs to come in the future.

### Examples

* One IP: `chickadee --ips 1.1.1.1`
* Select a few columns: `chickadee --ips 1.1.1.1 --columns query,city,country`
* Multiple IPs: `chickadee --ips "1.1.1.1,2.2.2.2 3.3.3.3    4.4.4.4"`
  * Note: as long as they are delimited, they are likely to be detected and resolved.
* A plain text file containing IPs: `chickadee --ips firewall.log`
* A gzip file containing IPs: `chickadee --ips cloudtrail.log.gz`

## Development

## Testing

You can run tests with `cargo test`. You may want to use `cargo watch -x test`
to continuously run tests as you work.

### CI

The CI will automatically run 2 tests, seen in detail in `.github/workflows`:

1. `cargo checkmate`
2. `cargo run -- ...` with 3 scenarios
   1. A single IP, with a set of columns
   2. A file of IPs, with a set of columns
   3. A single IP with all columns

### Sending SBOM to dependency track

Run the `./tools/post-bom.sh` script from the same directory as this README with the args:

1. API Endpoint, ie. `http://localhost:8081`
2. API Key
3. UUID of the project in dependency track

### Running profiling

1. Install perf
2. Install flamegraph, `cargo install flamegraph`
3. Build the binary in debug mode, `cargo build`
4. Run the tool with the args, `flamegraph -- target/debug/chickadee --ips test_data/test.ips.txt --columns query,lat,lon`
