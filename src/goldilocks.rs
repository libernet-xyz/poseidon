use crate::{params::decode_constants, poseidon};
use starkom_ff::Field;
use starkom_goldilocks::GL as Scalar;
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
    use super::*;
    use starkom_goldilocks::from_const;

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

    fn parse_scalar(s: &'static str) -> Scalar {
        s.parse().unwrap()
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
    fn test_hash_t12_8() {
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
            ]),
            [
                parse_scalar("0xe14b6d896101a5b4"),
                parse_scalar("0x1aac9183d60f237e"),
                parse_scalar("0x68b940c9c47cff6a"),
                parse_scalar("0x6b1788ed1bea5707"),
                parse_scalar("0x60b5cad71c7370b1"),
                parse_scalar("0xaa56bf27f39687ab"),
                parse_scalar("0x67227b2658e6ad5f"),
                parse_scalar("0xbf1bb3b67213367b"),
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
            ]),
            parse_scalar("0xe14b6d896101a5b4")
        );
    }

    #[test]
    fn test_hash_t12_9() {
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
            ]),
            [
                parse_scalar("0xc63a0bf2f95303e1"),
                parse_scalar("0x86a0748d2e82d798"),
                parse_scalar("0xa1cc64de6ee46e7c"),
                parse_scalar("0xd04bef1710a62ad1"),
                parse_scalar("0xdd322535c9196309"),
                parse_scalar("0x6a1bf401fe051af9"),
                parse_scalar("0x4a36ccc9e9ca943e"),
                parse_scalar("0x4a295ffdbefde65e"),
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
            ]),
            parse_scalar("0xc63a0bf2f95303e1")
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

    #[test]
    fn test_hash_t16_1() {
        assert_eq!(
            hash_t16([from_const(42)]),
            [
                parse_scalar("0xdef41b29f66be8e9"),
                parse_scalar("0x1fba73cdb43e8462"),
                parse_scalar("0x1a01f1f276b56991"),
                parse_scalar("0x164317b1733e367e"),
                parse_scalar("0x7b74b258c0154f70"),
                parse_scalar("0x10bcb73a61e00f59"),
                parse_scalar("0xee9c3dec56608a3b"),
                parse_scalar("0xaa0e2c8599445a94"),
                parse_scalar("0xc496b09c494bb04e"),
                parse_scalar("0xfe4a3d780673044e"),
                parse_scalar("0x5a4ec940f47892d8"),
                parse_scalar("0x8a9865254a43ca15"),
            ]
        );
        assert_eq!(
            hash_t16_0([from_const(42)]),
            parse_scalar("0xdef41b29f66be8e9")
        );
    }

    #[test]
    fn test_hash_t16_2() {
        assert_eq!(
            hash_t16([from_const(12), from_const(34)]),
            [
                parse_scalar("0xc999a3dbb04968fd"),
                parse_scalar("0xb0d3c251c8388ee2"),
                parse_scalar("0xa0bc16d1c156b276"),
                parse_scalar("0x2ed71bf002f547dc"),
                parse_scalar("0x846d6d5c2ad6f58a"),
                parse_scalar("0x2b32c04d95dd2aca"),
                parse_scalar("0x566bd1b3a761a7df"),
                parse_scalar("0x1e51af6aac25cc9e"),
                parse_scalar("0x230b7b131623a999"),
                parse_scalar("0xd19e96fc5f055752"),
                parse_scalar("0x6763ff9f8e72dce0"),
                parse_scalar("0x590b510a29dce54f"),
            ]
        );
        assert_eq!(
            hash_t16_0([from_const(12), from_const(34)]),
            parse_scalar("0xc999a3dbb04968fd")
        );
    }

    #[test]
    fn test_hash_t16_12() {
        assert_eq!(
            hash_t16([
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
                parse_scalar("0xf179baf40ae0c330"),
                parse_scalar("0x79acb9df598de251"),
                parse_scalar("0x6df530819916cd08"),
                parse_scalar("0x2ee25f32b41294e7"),
                parse_scalar("0x725309fdbb2ab816"),
                parse_scalar("0x432dde511a7d22d6"),
                parse_scalar("0xcc0174751bc138ef"),
                parse_scalar("0x03a221860ca1fbed"),
                parse_scalar("0xdc040277db765a3f"),
                parse_scalar("0xe1992a8d7238dcc3"),
                parse_scalar("0x4d10f284ce6d37ec"),
                parse_scalar("0x7623bc1e2d725986"),
            ]
        );
        assert_eq!(
            hash_t16_0([
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
            parse_scalar("0xf179baf40ae0c330")
        );
    }

    #[test]
    fn test_hash_t16_13() {
        assert_eq!(
            hash_t16([
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
                parse_scalar("0x6e938cd6380ca309"),
                parse_scalar("0x42003fc7ea25b80e"),
                parse_scalar("0xbd453d7d636fdfd3"),
                parse_scalar("0xa826915523eb2e8a"),
                parse_scalar("0x1a0557fa0fcd083c"),
                parse_scalar("0xc3a01de5a30b18bc"),
                parse_scalar("0xdb4ccdaf392c9e5b"),
                parse_scalar("0x6aadf34d2d074124"),
                parse_scalar("0x08c66354752c0b74"),
                parse_scalar("0xaf2e2fa49cd35c16"),
                parse_scalar("0x2cd1cabe05c14bf4"),
                parse_scalar("0x18e9ee123483b864"),
            ]
        );
        assert_eq!(
            hash_t16_0([
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
            parse_scalar("0x6e938cd6380ca309")
        );
    }

    #[test]
    fn test_hash_t16_15() {
        assert_eq!(
            hash_t16([
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
            ]),
            [
                parse_scalar("0xf1ffabeb4d336bba"),
                parse_scalar("0x261e811603bfe28d"),
                parse_scalar("0x75b5e4670d0481e6"),
                parse_scalar("0xe1ff9f44f62748af"),
                parse_scalar("0x0e9dede7e88dcc4a"),
                parse_scalar("0x058947103f27a11a"),
                parse_scalar("0x88c4c689ba7011a6"),
                parse_scalar("0x0a419f1668336096"),
                parse_scalar("0x0a64b4a121ccde16"),
                parse_scalar("0xfeff6521fddaa71a"),
                parse_scalar("0x439ab9ccc53f1ddf"),
                parse_scalar("0x875c7f7945195f3f"),
            ]
        );
        assert_eq!(
            hash_t16_0([
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
            ]),
            parse_scalar("0xf1ffabeb4d336bba")
        );
    }

    #[test]
    fn test_hash_t16_16() {
        assert_eq!(
            hash_t16([
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
                parse_scalar("0xc288b10706f3d38b"),
                parse_scalar("0x02355c943a749e0e"),
                parse_scalar("0x4a9dd32ed194928c"),
                parse_scalar("0x681aa9f04ff6db0e"),
                parse_scalar("0x0d87155e55e364e5"),
                parse_scalar("0x5e7abf7c07d1a0dd"),
                parse_scalar("0xb0edd99cb54033ef"),
                parse_scalar("0xb1c724587a8e7c31"),
                parse_scalar("0x4d8f880375c5a796"),
                parse_scalar("0x84c4c0dda293c695"),
                parse_scalar("0x4869edbe707edbfe"),
                parse_scalar("0x966fef8312357e2f"),
            ]
        );
        assert_eq!(
            hash_t16_0([
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
            parse_scalar("0xc288b10706f3d38b")
        );
    }

    #[test]
    fn test_hash_t16_17() {
        assert_eq!(
            hash_t16([
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
                from_const(16),
            ]),
            [
                parse_scalar("0x47af4b6d88d07ed1"),
                parse_scalar("0xc2f19dbbad49a0f9"),
                parse_scalar("0x6ffbfb25231e66a2"),
                parse_scalar("0x124cbdc1618b03b7"),
                parse_scalar("0x3beaa20070a4f603"),
                parse_scalar("0x1f768f85230e0aa1"),
                parse_scalar("0xe2353ab4be1dedf4"),
                parse_scalar("0x0464876681995543"),
                parse_scalar("0xe945e903b851b885"),
                parse_scalar("0xd794f3c95d9aa257"),
                parse_scalar("0x0d446645d8d1bc68"),
                parse_scalar("0x8d5aa6d3a3d4e158"),
            ]
        );
        assert_eq!(
            hash_t16_0([
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
                from_const(16),
            ]),
            parse_scalar("0x47af4b6d88d07ed1")
        );
    }
}
