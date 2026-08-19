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

    #[test]
    fn test_hash_t12_1() {
        assert_eq!(
            hash_t12([from_const(42)]),
            [
                parse_scalar("0x6a57b5ece3566dbd"),
                parse_scalar("0x37fecc16d9cfe4d8"),
                parse_scalar("0x87a6f363956fe2b2"),
                parse_scalar("0xf868588f9473b0bd"),
                parse_scalar("0x20d0582daff6ee91"),
                parse_scalar("0xa47bbed413e1692f"),
                parse_scalar("0xc6b92123fcbe019f"),
                parse_scalar("0x40269b7cf3e1b638"),
            ]
        );
        assert_eq!(
            hash_t12_0([from_const(42)]),
            parse_scalar("0x6a57b5ece3566dbd")
        );
    }

    #[test]
    fn test_hash_t12_2() {
        assert_eq!(
            hash_t12([from_const(12), from_const(34)]),
            [
                parse_scalar("0x6445625f0d056add"),
                parse_scalar("0xa1935d6bb9a21dbd"),
                parse_scalar("0xb7ad443e6e6c0675"),
                parse_scalar("0x01830bfa9674e5d2"),
                parse_scalar("0x5102f2ef5339983a"),
                parse_scalar("0xdf2a11477a14be6a"),
                parse_scalar("0x51799229355474e5"),
                parse_scalar("0x3196d1186580870c"),
            ]
        );
        assert_eq!(
            hash_t12_0([from_const(12), from_const(34)]),
            parse_scalar("0x6445625f0d056add")
        );
    }

    #[test]
    fn test_hash_t12_11() {
        assert_eq!(
            hash_t12([
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
            ]),
            [
                parse_scalar("0xf8a00ff02e42eed3"),
                parse_scalar("0xd6b1daed539049fd"),
                parse_scalar("0x33d38df121b32169"),
                parse_scalar("0x7a3afc8b0108ed01"),
                parse_scalar("0xa8c34cf21675d116"),
                parse_scalar("0xa093820693b8bf4d"),
                parse_scalar("0x7ecb473d29cd20f6"),
                parse_scalar("0xec4e1f8d6c05b503"),
            ]
        );
        assert_eq!(
            hash_t12_0([
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
            ]),
            parse_scalar("0xf8a00ff02e42eed3")
        );
    }

    #[test]
    fn test_hash_t12_12() {
        assert_eq!(
            hash_t12([
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
                parse_scalar("0x10b2f4e01b0aec60"),
                parse_scalar("0x8cfd4fad5c1485b1"),
                parse_scalar("0xd42d72404ff29608"),
                parse_scalar("0x9979d7f933c4c9b7"),
                parse_scalar("0x2c622ee5d670b22a"),
                parse_scalar("0x8939341f1dbda787"),
                parse_scalar("0xdb13990bc23fde30"),
                parse_scalar("0x0b4fbefad1b2045a"),
            ]
        );
        assert_eq!(
            hash_t12_0([
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
            parse_scalar("0x10b2f4e01b0aec60")
        );
    }

    #[test]
    fn test_hash_t12_13() {
        assert_eq!(
            hash_t12([
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
            ]),
            [
                parse_scalar("0xa6a699fba5931f63"),
                parse_scalar("0xae9d3baacfcb098d"),
                parse_scalar("0xa4492452ed96f301"),
                parse_scalar("0xae3506dfa566f967"),
                parse_scalar("0xd5ae1af4d3bf25f2"),
                parse_scalar("0x5f52fff18bf88108"),
                parse_scalar("0xf62e1b00419659dc"),
                parse_scalar("0x5e224b7108c1df27"),
            ]
        );
        assert_eq!(
            hash_t12_0([
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
            ]),
            parse_scalar("0xa6a699fba5931f63")
        );
    }

    // TODO
}
