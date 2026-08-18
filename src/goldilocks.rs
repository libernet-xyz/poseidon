use crate::{params::decode_constants, poseidon};
use starkom_ff::Field;
use starkom_goldilocks::Scalar;
use std::sync::LazyLock;

/// Poseidon configuration for the Goldilocks field.
pub struct GoldilocksConfig<const T: usize> {}

impl poseidon::Config<Scalar, 12> for GoldilocksConfig<12> {
    fn num_full_rounds() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        22
    }

    fn sbox(x: Scalar) -> Scalar {
        x.cube().square() * x
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 360]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/goldilocks/arc_t12.bin");
            decode_constants::<Scalar, 360>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 144]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/goldilocks/mds_t12.bin");
            decode_constants::<Scalar, 144>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 16> for GoldilocksConfig<16> {
    fn num_full_rounds() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        22
    }

    fn sbox(x: Scalar) -> Scalar {
        x.cube().square() * x
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 480]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/goldilocks/arc_t16.bin");
            decode_constants::<Scalar, 480>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 256]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/goldilocks/mds_t16.bin");
            decode_constants::<Scalar, 256>(bytes)
        });
        &*MATRIX
    }
}

/// Poseidon configuration for Goldilocks with T=12.
pub type GoldilocksConfig12 = GoldilocksConfig<12>;

/// Poseidon configuration for Goldilocks with T=16.
pub type GoldilocksConfig16 = GoldilocksConfig<16>;

#[cfg(test)]
mod tests {
    use starkom_goldilocks::{from_const, parse_scalar};

    use super::*;

    fn hash_t12(inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 8] {
        poseidon::hash::<GoldilocksConfig12, Scalar, 12, 8, 4>(inputs)
    }

    fn hash_t12_0(inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<GoldilocksConfig12, Scalar, 12, 8, 4>(inputs)
    }

    fn hash_t16(inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 12] {
        poseidon::hash::<GoldilocksConfig16, Scalar, 16, 12, 4>(inputs)
    }

    fn hash_t16_0(inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<GoldilocksConfig16, Scalar, 16, 12, 4>(inputs)
    }

    #[test]
    fn test_permutation_t12() {
        assert_eq!(
            poseidon::permutation::<GoldilocksConfig12, Scalar, 12>([
                from_const(0),
                from_const(1),
                from_const(2),
                from_const(3),
                from_const(4),
                from_const(5),
                from_const(6),
                from_const(7),
                from_const(8),
                from_const(9),
                from_const(10),
                from_const(11),
            ]),
            [
                parse_scalar("0x056bda38ad308e78"),
                parse_scalar("0x1f38944238b8ccd0"),
                parse_scalar("0x80bef63a171f3156"),
                parse_scalar("0x27bbc645b2a3198c"),
                parse_scalar("0x9befae3f221509b3"),
                parse_scalar("0xa1cfa54ae2c44c9e"),
                parse_scalar("0xa1c876869f1c52f8"),
                parse_scalar("0x7ffa21471eff65af"),
                parse_scalar("0xdc565450ad52b99e"),
                parse_scalar("0x4b8b1daf8e8ea3c6"),
                parse_scalar("0xf866b42495e61984"),
                parse_scalar("0x7af57b5f91f196fe"),
            ]
        );
    }

    #[test]
    fn test_permutation_t16() {
        assert_eq!(
            poseidon::permutation::<GoldilocksConfig16, Scalar, 16>([
                from_const(0),
                from_const(1),
                from_const(2),
                from_const(3),
                from_const(4),
                from_const(5),
                from_const(6),
                from_const(7),
                from_const(8),
                from_const(9),
                from_const(10),
                from_const(11),
                from_const(12),
                from_const(13),
                from_const(14),
                from_const(15),
            ]),
            [
                parse_scalar("0x6a84bf02be1f328d"),
                parse_scalar("0xec14d274b936a21a"),
                parse_scalar("0xc0539d7bd4eb66de"),
                parse_scalar("0xb317ecf41fa8d55b"),
                parse_scalar("0x80b0d36f66671f8a"),
                parse_scalar("0x74a1592b9a16e832"),
                parse_scalar("0x65e53afadfadc8c3"),
                parse_scalar("0xa0007e5ee96ee4b2"),
                parse_scalar("0x6dd5661a877003a8"),
                parse_scalar("0xc36a09c2dc25cd6e"),
                parse_scalar("0xcbda3d58f7cf85f4"),
                parse_scalar("0x34cb1d63c35596cf"),
                parse_scalar("0x4fcd09b24769e281"),
                parse_scalar("0x6c514f906998c65d"),
                parse_scalar("0xc447035d8d71952b"),
                parse_scalar("0x591863454267826f"),
            ]
        );
    }

    // TODO
}
