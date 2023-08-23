# cosmos-sdk-proto

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Rust crate for interacting with [Protobufs] defined by the [Cosmos SDK].

The goal of this crate is to provide complete proto struct definitions for interacting
with a Cosmos SDK blockchain.

Currently, this crate only provides a subset of the many total structs exported by
Cosmos SDK proto files.

Pull requests to expand coverage are welcome.

[Documentation][docs-link]

## Minimum Supported Rust Version

This crate is supported on Rust **1.56** or newer.

[//]: # "badges"
[crate-image]: https://img.shields.io/crates/v/cosmos-sdk-proto.svg?logo=rust
[crate-link]: https://crates.io/crates/cosmos-sdk-proto
[docs-image]: https://docs.rs/cosmos-sdk-proto/badge.svg
[docs-link]: https://docs.rs/cosmos-sdk-proto/
[build-image]: https://github.com/cosmos/cosmos-rust/workflows/cosmos-sdk-proto/badge.svg
[build-link]: https://github.com/cosmos/cosmos-rust/actions/workflows/cosmos-sdk-proto.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/cosmos/cosmos-rust/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg

[//]: # "general links"
[Protobufs]: (https://github.com/cosmos/cosmos-sdk/tree/master/proto/)
[Cosmos SDK]: https://github.com/cosmos/cosmos-sdk

## Issues with the Cosmos Staking module prost file

Unfortunately (the upstream source for the CosmosSDK staking module proto definitions)[https://github.com/cosmos/cosmos-sdk/blob/v0.45.16/proto/cosmos/staking/v1beta1/authz.proto#L20-L30] causes a namespace conflict when using prost.
Particularly there is a `oneof` named `validators` which becomes a Rust `enum` named `Validators` and a `message` named `Validators` which becomes a Rust `struct` named `Validators`, which causes a failure to compile.

Whenever this file is updated it has been manually fixed to rename the `enum` to `ValidatorsEnum` and the relative disuse of the StakingAuthorization types means that is OK for now. If you run proto_build and see the `src/prost/cosmos.staking.v1beta1.rs` file has changed in insignificant ways, it is fine to ignore that file.