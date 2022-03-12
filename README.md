# u128

U128 helpers for Solana programs.

[![Crates.io](https://img.shields.io/crates/v/u128)](https://crates.io/crates/u128)
[![License](https://img.shields.io/crates/l/u128)](https://github.com/saber-hq/u128/blob/master/LICENSE.txt)
[![Build Status](https://img.shields.io/github/workflow/status/saber-hq/u128/Rust/master)](https://github.com/saber-hq/u128/actions/workflows/rust.yml?query=branch%3Amaster)
[![Contributors](https://img.shields.io/github/contributors/saber-hq/u128)](https://github.com/saber-hq/u128/graphs/contributors)

## Motivation

U128 division is [very inefficient](https://github.com/solana-labs/solana/issues/19549) on
Solana BPF. This crate exposes the [uint] crate as a stopgap.

License: Apache-2.0
