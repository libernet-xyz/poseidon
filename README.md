# Poseidon

[![CI](https://img.shields.io/github/actions/workflow/status/libernet-xyz/poseidon/ci.yml?label=CI)](https://github.com/libernet-xyz/poseidon/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/starkom-poseidon)](https://crates.io/crates/starkom-poseidon)
[![license](https://img.shields.io/crates/l/starkom-poseidon)](https://github.com/libernet-xyz/poseidon/blob/main/LICENSE)

## Overview

This is Starkom's implementation of the [Poseidon algebraic hash](https://eprint.iacr.org/2019/458).

The implementation is generic and works on any prime field.

Configurations for the BLS12-381 and BlueSky prime fields are provided; they support T=3 (R=2, C=1)
and T=4 (R=3, C=1).

> [!NOTE]
> The BLS12-381 configurations are controlled by the `bls12_381` feature flag, which is disabled by
> default.

## Usage

The following example functions instantiate Poseidon with T=3 and T=4 respectively, squeezing a
single element from the output. Both use a single element for capacity.

```rs
use starkom_bluesky::Scalar;
use starkom_poseidon;

fn hash_t3(inputs: &[Scalar]) -> Scalar {
    starkom_poseidon::hash::<starkom_poseidon::bluesky::BlueSkyConfig3, Scalar, 3, 2, 1>(inputs)[0]
}

fn hash_t4(inputs: &[Scalar]) -> Scalar {
    starkom_poseidon::hash::<starkom_poseidon::bluesky::BlueSkyConfig4, Scalar, 4, 3, 1>(inputs)[0]
}
```
