# OPDS (de)serialization types

[![Build Status](https://github.com/ulyssa/opds-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ulyssa/opds-rs/actions?query=workflow%3ACI+)
[![License: Apache 2.0](https://img.shields.io/crates/l/opds.svg?logo=apache)](https://crates.io/crates/opds)
[![Latest Version](https://img.shields.io/crates/v/opds.svg?logo=rust)](https://crates.io/crates/opds)
[![Docs Status](https://docs.rs/opds/badge.svg)](https://docs.rs/crate/opds/)

## About

This is a Rust library for serializing and deserializing OPDS feeds.

Currently, it only supports [the v2.0 draft specification][opds-spec-v2].

## Usage

This crate is [on crates.io](https://crates.io/crates/opds) and can be
used by adding `opds` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
opds = "*"
```

## License

`opds-rs` is licensed under either of [Apache License, Version 2.0] or
[MIT license] at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions. 

[Apache License, Version 2.0]: https://github.com/ulyssa/opds-rs/blob/master/LICENSE-APACHE
[MIT license]: https://github.com/ulyssa/opds-rs/blob/master/LICENSE-MIT
[opds-spec-v2]: https://drafts.opds.io/opds-2.0.html
