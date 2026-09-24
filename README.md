# Poseidon

[![CI](https://img.shields.io/github/actions/workflow/status/libernet-xyz/poseidon/ci.yml?label=CI)](https://github.com/libernet-xyz/poseidon/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/starkom-poseidon)](https://crates.io/crates/starkom-poseidon)
[![license](https://img.shields.io/crates/l/starkom-poseidon)](https://github.com/libernet-xyz/poseidon/blob/main/LICENSE)

## Overview

This is Starkom's implementation of the [Poseidon algebraic hash](https://eprint.iacr.org/2019/458).

Note that this crate uses version 1 of the permutation. For version 2 see the
[`starkom-poseidon2`](https://crates.io/crates/starkom-poseidon2) crate.

The implementation is generic and works on any prime field.

Configurations for the BLS12-381, Goldilocks, KoalaBear,
[BlueSky](https://crates.io/crates/starkom-bluesky), and
[Schraderbrau](https://crates.io/crates/starkom-schraderbrau) prime fields are provided. The
BLS12-381, BlueSky, and Schraderbrau configurations support T=3 and T=4; the Goldilocks
configurations support T=12 and T=16; the KoalaBear configurations support T=24 and T=32.

> [!NOTE]
> All predefined configurations are gated behind feature flags to avoid including all constants in
> all builds. The currently defined feature flags are `bls12_381`, `goldilocks`, `koalabear`,
> `bluesky`, and `schraderbrau`, all disabled by default.

## Usage

The following example functions instantiate Poseidon with T=3 and T=4 respectively, squeezing a
single element from the output. Both use a single element for capacity, initialized with a domain
separator tag (the `dst` parameter).

```rs
use starkom_bluesky::Scalar;
use starkom_poseidon;

fn hash_t3(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
    starkom_poseidon::hash::<starkom_poseidon::bluesky::BlueSkyConfig3, Scalar, 3, 2, 1>([dst], inputs)[0]
}

fn hash_t4(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
    starkom_poseidon::hash::<starkom_poseidon::bluesky::BlueSkyConfig4, Scalar, 4, 3, 1>([dst], inputs)[0]
}
```
