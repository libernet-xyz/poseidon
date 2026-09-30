use crate::params::decode_constants;
use crate::poseidon;
use starkom_goldilocks::GL as Scalar;
use std::sync::LazyLock;

/// Poseidon configuration for the Goldilocks field.
pub struct GoldilocksConfig<const T: usize> {}

impl poseidon::Config<Scalar, 12> for GoldilocksConfig<12> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        22
    }

    fn alpha() -> usize {
        7
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
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        22
    }

    fn alpha() -> usize {
        7
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

impl poseidon::Config<Scalar, 20> for GoldilocksConfig<20> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        22
    }

    fn alpha() -> usize {
        7
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 600]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/goldilocks/arc_t20.bin");
            decode_constants::<Scalar, 600>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 400]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/goldilocks/mds_t20.bin");
            decode_constants::<Scalar, 400>(bytes)
        });
        &*MATRIX
    }
}

/// Poseidon configuration for Goldilocks with T=12.
pub type GoldilocksConfig12 = GoldilocksConfig<12>;

/// Poseidon configuration for Goldilocks with T=16.
pub type GoldilocksConfig16 = GoldilocksConfig<16>;

/// Poseidon configuration for Goldilocks with T=20.
pub type GoldilocksConfig20 = GoldilocksConfig<20>;

#[cfg(test)]
mod tests {
    use super::*;
    use starkom_ff::Field;
    use starkom_goldilocks::from_const;

    const DST: [Scalar; 4] = [Scalar::ZERO; 4];

    fn hash_t12(dst: [Scalar; 4], inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 8] {
        poseidon::hash::<GoldilocksConfig12, Scalar, 12, 8, 4>(dst, inputs)
    }

    fn hash_t12_0(dst: [Scalar; 4], inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<GoldilocksConfig12, Scalar, 12, 8, 4>(dst, inputs)
    }

    fn hash_t16(dst: [Scalar; 4], inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 12] {
        poseidon::hash::<GoldilocksConfig16, Scalar, 16, 12, 4>(dst, inputs)
    }

    fn hash_t16_0(dst: [Scalar; 4], inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<GoldilocksConfig16, Scalar, 16, 12, 4>(dst, inputs)
    }

    fn hash_t20(dst: [Scalar; 4], inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 16] {
        poseidon::hash::<GoldilocksConfig20, Scalar, 20, 16, 4>(dst, inputs)
    }

    fn hash_t20_0(dst: [Scalar; 4], inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<GoldilocksConfig20, Scalar, 20, 16, 4>(dst, inputs)
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
                from_const(0x056bda38ad308e78),
                from_const(0x1f38944238b8ccd0),
                from_const(0x80bef63a171f3156),
                from_const(0x27bbc645b2a3198c),
                from_const(0x9befae3f221509b3),
                from_const(0xa1cfa54ae2c44c9e),
                from_const(0xa1c876869f1c52f8),
                from_const(0x7ffa21471eff65af),
                from_const(0xdc565450ad52b99e),
                from_const(0x4b8b1daf8e8ea3c6),
                from_const(0xf866b42495e61984),
                from_const(0x7af57b5f91f196fe),
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
                from_const(0x6a84bf02be1f328d),
                from_const(0xec14d274b936a21a),
                from_const(0xc0539d7bd4eb66de),
                from_const(0xb317ecf41fa8d55b),
                from_const(0x80b0d36f66671f8a),
                from_const(0x74a1592b9a16e832),
                from_const(0x65e53afadfadc8c3),
                from_const(0xa0007e5ee96ee4b2),
                from_const(0x6dd5661a877003a8),
                from_const(0xc36a09c2dc25cd6e),
                from_const(0xcbda3d58f7cf85f4),
                from_const(0x34cb1d63c35596cf),
                from_const(0x4fcd09b24769e281),
                from_const(0x6c514f906998c65d),
                from_const(0xc447035d8d71952b),
                from_const(0x591863454267826f),
            ]
        );
    }

    #[test]
    fn test_permutation_t20() {
        assert_eq!(
            poseidon::permutation::<GoldilocksConfig20, Scalar, 20>([
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
                from_const(17),
                from_const(18),
                from_const(19),
            ]),
            [
                from_const(0x082a507bd86740b3),
                from_const(0xcae7f445392dd5fd),
                from_const(0x7e09cdb1244aebae),
                from_const(0x3456ec275aa454e3),
                from_const(0x865849dd9341049f),
                from_const(0xee828d5114d9c459),
                from_const(0x2c41d4f929d60a89),
                from_const(0x435a9c2ac9a32e09),
                from_const(0xd40ef139175f4295),
                from_const(0xaf69f572ad791e78),
                from_const(0x9e9536888849971e),
                from_const(0xe84df78c71ce6cf8),
                from_const(0x6530aa5ab6cc8ea4),
                from_const(0xb57c19c935de4546),
                from_const(0xdb6a1ca8597892b9),
                from_const(0x05956742fd5f6238),
                from_const(0xbfc99dd5cc2cf88b),
                from_const(0xdf6582e2c16b673b),
                from_const(0xe96936cf01d8ca52),
                from_const(0x87b21656a35c348d),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t12() {
        assert_eq!(
            hash_t12(
                DST,
                [
                    from_const(0),
                    from_const(1),
                    from_const(2),
                    from_const(3),
                    from_const(4),
                    from_const(5),
                    from_const(6),
                    from_const(7),
                ]
            ),
            [
                from_const(0xe14b6d896101a5b4),
                from_const(0x1aac9183d60f237e),
                from_const(0x68b940c9c47cff6a),
                from_const(0x6b1788ed1bea5707),
                from_const(0x60b5cad71c7370b1),
                from_const(0xaa56bf27f39687ab),
                from_const(0x67227b2658e6ad5f),
                from_const(0xbf1bb3b67213367b),
            ]
        );
        assert_eq!(
            hash_t12(
                [from_const(8), from_const(9), from_const(10), from_const(11)],
                [
                    from_const(0),
                    from_const(1),
                    from_const(2),
                    from_const(3),
                    from_const(4),
                    from_const(5),
                    from_const(6),
                    from_const(7),
                ]
            ),
            [
                from_const(0x056bda38ad308e78),
                from_const(0x1f38944238b8ccd0),
                from_const(0x80bef63a171f3156),
                from_const(0x27bbc645b2a3198c),
                from_const(0x9befae3f221509b3),
                from_const(0xa1cfa54ae2c44c9e),
                from_const(0xa1c876869f1c52f8),
                from_const(0x7ffa21471eff65af),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t16() {
        assert_eq!(
            hash_t16(
                DST,
                [
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
                ]
            ),
            [
                from_const(0xf179baf40ae0c330),
                from_const(0x79acb9df598de251),
                from_const(0x6df530819916cd08),
                from_const(0x2ee25f32b41294e7),
                from_const(0x725309fdbb2ab816),
                from_const(0x432dde511a7d22d6),
                from_const(0xcc0174751bc138ef),
                from_const(0x03a221860ca1fbed),
                from_const(0xdc040277db765a3f),
                from_const(0xe1992a8d7238dcc3),
                from_const(0x4d10f284ce6d37ec),
                from_const(0x7623bc1e2d725986),
            ]
        );
        assert_eq!(
            hash_t16(
                [
                    from_const(12),
                    from_const(13),
                    from_const(14),
                    from_const(15)
                ],
                [
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
                ]
            ),
            [
                from_const(0x6a84bf02be1f328d),
                from_const(0xec14d274b936a21a),
                from_const(0xc0539d7bd4eb66de),
                from_const(0xb317ecf41fa8d55b),
                from_const(0x80b0d36f66671f8a),
                from_const(0x74a1592b9a16e832),
                from_const(0x65e53afadfadc8c3),
                from_const(0xa0007e5ee96ee4b2),
                from_const(0x6dd5661a877003a8),
                from_const(0xc36a09c2dc25cd6e),
                from_const(0xcbda3d58f7cf85f4),
                from_const(0x34cb1d63c35596cf),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t20() {
        assert_eq!(
            hash_t20(
                DST,
                [
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
                ]
            ),
            [
                from_const(0xe9bc8f5856d8020f),
                from_const(0x2926b3f41dbc0904),
                from_const(0xf3865d23d15dbffa),
                from_const(0x3411a90a6c3ffcc7),
                from_const(0x693e7dcc81462529),
                from_const(0x0b66e3b30014f3a5),
                from_const(0x63bc2540ceb6e68d),
                from_const(0x540b66853c62c9e8),
                from_const(0xa4f9728000273c70),
                from_const(0xc57933c69abbf5a9),
                from_const(0x9ab6fdb2d805bd59),
                from_const(0x7a6eed835545aaf2),
                from_const(0x74d9df4b6f397c63),
                from_const(0xbd7ff160d047e569),
                from_const(0x8e59c41861615d1e),
                from_const(0x7d26431a8b73f106),
            ]
        );
        assert_eq!(
            hash_t20(
                [
                    from_const(16),
                    from_const(17),
                    from_const(18),
                    from_const(19)
                ],
                [
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
                ]
            ),
            [
                from_const(0x082a507bd86740b3),
                from_const(0xcae7f445392dd5fd),
                from_const(0x7e09cdb1244aebae),
                from_const(0x3456ec275aa454e3),
                from_const(0x865849dd9341049f),
                from_const(0xee828d5114d9c459),
                from_const(0x2c41d4f929d60a89),
                from_const(0x435a9c2ac9a32e09),
                from_const(0xd40ef139175f4295),
                from_const(0xaf69f572ad791e78),
                from_const(0x9e9536888849971e),
                from_const(0xe84df78c71ce6cf8),
                from_const(0x6530aa5ab6cc8ea4),
                from_const(0xb57c19c935de4546),
                from_const(0xdb6a1ca8597892b9),
                from_const(0x05956742fd5f6238),
            ]
        );
    }

    #[test]
    fn test_hash_t12_1() {
        assert_eq!(
            hash_t12(DST, [from_const(42)]),
            [
                from_const(0x6a57b5ece3566dbd),
                from_const(0x37fecc16d9cfe4d8),
                from_const(0x87a6f363956fe2b2),
                from_const(0xf868588f9473b0bd),
                from_const(0x20d0582daff6ee91),
                from_const(0xa47bbed413e1692f),
                from_const(0xc6b92123fcbe019f),
                from_const(0x40269b7cf3e1b638),
            ]
        );
        assert_eq!(
            hash_t12_0(DST, [from_const(42)]),
            from_const(0x6a57b5ece3566dbd)
        );
    }

    #[test]
    fn test_hash_t12_2() {
        assert_eq!(
            hash_t12(DST, [from_const(12), from_const(34)]),
            [
                from_const(0x6445625f0d056add),
                from_const(0xa1935d6bb9a21dbd),
                from_const(0xb7ad443e6e6c0675),
                from_const(0x01830bfa9674e5d2),
                from_const(0x5102f2ef5339983a),
                from_const(0xdf2a11477a14be6a),
                from_const(0x51799229355474e5),
                from_const(0x3196d1186580870c),
            ]
        );
        assert_eq!(
            hash_t12_0(DST, [from_const(12), from_const(34)]),
            from_const(0x6445625f0d056add)
        );
    }

    #[test]
    fn test_hash_t12_8() {
        assert_eq!(
            hash_t12(
                DST,
                [
                    from_const(0),
                    from_const(1),
                    from_const(2),
                    from_const(3),
                    from_const(4),
                    from_const(5),
                    from_const(6),
                    from_const(7),
                ]
            ),
            [
                from_const(0xe14b6d896101a5b4),
                from_const(0x1aac9183d60f237e),
                from_const(0x68b940c9c47cff6a),
                from_const(0x6b1788ed1bea5707),
                from_const(0x60b5cad71c7370b1),
                from_const(0xaa56bf27f39687ab),
                from_const(0x67227b2658e6ad5f),
                from_const(0xbf1bb3b67213367b),
            ]
        );
        assert_eq!(
            hash_t12_0(
                DST,
                [
                    from_const(0),
                    from_const(1),
                    from_const(2),
                    from_const(3),
                    from_const(4),
                    from_const(5),
                    from_const(6),
                    from_const(7),
                ]
            ),
            from_const(0xe14b6d896101a5b4)
        );
    }

    #[test]
    fn test_hash_t12_9() {
        assert_eq!(
            hash_t12(
                DST,
                [
                    from_const(0),
                    from_const(1),
                    from_const(2),
                    from_const(3),
                    from_const(4),
                    from_const(5),
                    from_const(6),
                    from_const(7),
                    from_const(8),
                ]
            ),
            [
                from_const(0xc63a0bf2f95303e1),
                from_const(0x86a0748d2e82d798),
                from_const(0xa1cc64de6ee46e7c),
                from_const(0xd04bef1710a62ad1),
                from_const(0xdd322535c9196309),
                from_const(0x6a1bf401fe051af9),
                from_const(0x4a36ccc9e9ca943e),
                from_const(0x4a295ffdbefde65e),
            ]
        );
        assert_eq!(
            hash_t12_0(
                DST,
                [
                    from_const(0),
                    from_const(1),
                    from_const(2),
                    from_const(3),
                    from_const(4),
                    from_const(5),
                    from_const(6),
                    from_const(7),
                    from_const(8),
                ]
            ),
            from_const(0xc63a0bf2f95303e1)
        );
    }

    #[test]
    fn test_hash_t12_11() {
        assert_eq!(
            hash_t12(
                DST,
                [
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
                ]
            ),
            [
                from_const(0xf8a00ff02e42eed3),
                from_const(0xd6b1daed539049fd),
                from_const(0x33d38df121b32169),
                from_const(0x7a3afc8b0108ed01),
                from_const(0xa8c34cf21675d116),
                from_const(0xa093820693b8bf4d),
                from_const(0x7ecb473d29cd20f6),
                from_const(0xec4e1f8d6c05b503),
            ]
        );
        assert_eq!(
            hash_t12_0(
                DST,
                [
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
                ]
            ),
            from_const(0xf8a00ff02e42eed3)
        );
    }

    #[test]
    fn test_hash_t12_12() {
        assert_eq!(
            hash_t12(
                DST,
                [
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
                ]
            ),
            [
                from_const(0x10b2f4e01b0aec60),
                from_const(0x8cfd4fad5c1485b1),
                from_const(0xd42d72404ff29608),
                from_const(0x9979d7f933c4c9b7),
                from_const(0x2c622ee5d670b22a),
                from_const(0x8939341f1dbda787),
                from_const(0xdb13990bc23fde30),
                from_const(0x0b4fbefad1b2045a),
            ]
        );
        assert_eq!(
            hash_t12_0(
                DST,
                [
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
                ]
            ),
            from_const(0x10b2f4e01b0aec60)
        );
    }

    #[test]
    fn test_hash_t12_13() {
        assert_eq!(
            hash_t12(
                DST,
                [
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
                ]
            ),
            [
                from_const(0xa6a699fba5931f63),
                from_const(0xae9d3baacfcb098d),
                from_const(0xa4492452ed96f301),
                from_const(0xae3506dfa566f967),
                from_const(0xd5ae1af4d3bf25f2),
                from_const(0x5f52fff18bf88108),
                from_const(0xf62e1b00419659dc),
                from_const(0x5e224b7108c1df27),
            ]
        );
        assert_eq!(
            hash_t12_0(
                DST,
                [
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
                ]
            ),
            from_const(0xa6a699fba5931f63)
        );
    }

    #[test]
    fn test_hash_t16_1() {
        assert_eq!(
            hash_t16(DST, [from_const(42)]),
            [
                from_const(0xdef41b29f66be8e9),
                from_const(0x1fba73cdb43e8462),
                from_const(0x1a01f1f276b56991),
                from_const(0x164317b1733e367e),
                from_const(0x7b74b258c0154f70),
                from_const(0x10bcb73a61e00f59),
                from_const(0xee9c3dec56608a3b),
                from_const(0xaa0e2c8599445a94),
                from_const(0xc496b09c494bb04e),
                from_const(0xfe4a3d780673044e),
                from_const(0x5a4ec940f47892d8),
                from_const(0x8a9865254a43ca15),
            ]
        );
        assert_eq!(
            hash_t16_0(DST, [from_const(42)]),
            from_const(0xdef41b29f66be8e9)
        );
    }

    #[test]
    fn test_hash_t16_2() {
        assert_eq!(
            hash_t16(DST, [from_const(12), from_const(34)]),
            [
                from_const(0xc999a3dbb04968fd),
                from_const(0xb0d3c251c8388ee2),
                from_const(0xa0bc16d1c156b276),
                from_const(0x2ed71bf002f547dc),
                from_const(0x846d6d5c2ad6f58a),
                from_const(0x2b32c04d95dd2aca),
                from_const(0x566bd1b3a761a7df),
                from_const(0x1e51af6aac25cc9e),
                from_const(0x230b7b131623a999),
                from_const(0xd19e96fc5f055752),
                from_const(0x6763ff9f8e72dce0),
                from_const(0x590b510a29dce54f),
            ]
        );
        assert_eq!(
            hash_t16_0(DST, [from_const(12), from_const(34)]),
            from_const(0xc999a3dbb04968fd)
        );
    }

    #[test]
    fn test_hash_t16_12() {
        assert_eq!(
            hash_t16(
                DST,
                [
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
                ]
            ),
            [
                from_const(0xf179baf40ae0c330),
                from_const(0x79acb9df598de251),
                from_const(0x6df530819916cd08),
                from_const(0x2ee25f32b41294e7),
                from_const(0x725309fdbb2ab816),
                from_const(0x432dde511a7d22d6),
                from_const(0xcc0174751bc138ef),
                from_const(0x03a221860ca1fbed),
                from_const(0xdc040277db765a3f),
                from_const(0xe1992a8d7238dcc3),
                from_const(0x4d10f284ce6d37ec),
                from_const(0x7623bc1e2d725986),
            ]
        );
        assert_eq!(
            hash_t16_0(
                DST,
                [
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
                ]
            ),
            from_const(0xf179baf40ae0c330)
        );
    }

    #[test]
    fn test_hash_t16_13() {
        assert_eq!(
            hash_t16(
                DST,
                [
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
                ]
            ),
            [
                from_const(0x6e938cd6380ca309),
                from_const(0x42003fc7ea25b80e),
                from_const(0xbd453d7d636fdfd3),
                from_const(0xa826915523eb2e8a),
                from_const(0x1a0557fa0fcd083c),
                from_const(0xc3a01de5a30b18bc),
                from_const(0xdb4ccdaf392c9e5b),
                from_const(0x6aadf34d2d074124),
                from_const(0x08c66354752c0b74),
                from_const(0xaf2e2fa49cd35c16),
                from_const(0x2cd1cabe05c14bf4),
                from_const(0x18e9ee123483b864),
            ]
        );
        assert_eq!(
            hash_t16_0(
                DST,
                [
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
                ]
            ),
            from_const(0x6e938cd6380ca309)
        );
    }

    #[test]
    fn test_hash_t16_15() {
        assert_eq!(
            hash_t16(
                DST,
                [
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
                ]
            ),
            [
                from_const(0xf1ffabeb4d336bba),
                from_const(0x261e811603bfe28d),
                from_const(0x75b5e4670d0481e6),
                from_const(0xe1ff9f44f62748af),
                from_const(0x0e9dede7e88dcc4a),
                from_const(0x058947103f27a11a),
                from_const(0x88c4c689ba7011a6),
                from_const(0x0a419f1668336096),
                from_const(0x0a64b4a121ccde16),
                from_const(0xfeff6521fddaa71a),
                from_const(0x439ab9ccc53f1ddf),
                from_const(0x875c7f7945195f3f),
            ]
        );
        assert_eq!(
            hash_t16_0(
                DST,
                [
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
                ]
            ),
            from_const(0xf1ffabeb4d336bba)
        );
    }

    #[test]
    fn test_hash_t16_16() {
        assert_eq!(
            hash_t16(
                DST,
                [
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
                ]
            ),
            [
                from_const(0xc288b10706f3d38b),
                from_const(0x02355c943a749e0e),
                from_const(0x4a9dd32ed194928c),
                from_const(0x681aa9f04ff6db0e),
                from_const(0x0d87155e55e364e5),
                from_const(0x5e7abf7c07d1a0dd),
                from_const(0xb0edd99cb54033ef),
                from_const(0xb1c724587a8e7c31),
                from_const(0x4d8f880375c5a796),
                from_const(0x84c4c0dda293c695),
                from_const(0x4869edbe707edbfe),
                from_const(0x966fef8312357e2f),
            ]
        );
        assert_eq!(
            hash_t16_0(
                DST,
                [
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
                ]
            ),
            from_const(0xc288b10706f3d38b)
        );
    }

    #[test]
    fn test_hash_t16_17() {
        assert_eq!(
            hash_t16(
                DST,
                [
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
                ]
            ),
            [
                from_const(0x47af4b6d88d07ed1),
                from_const(0xc2f19dbbad49a0f9),
                from_const(0x6ffbfb25231e66a2),
                from_const(0x124cbdc1618b03b7),
                from_const(0x3beaa20070a4f603),
                from_const(0x1f768f85230e0aa1),
                from_const(0xe2353ab4be1dedf4),
                from_const(0x0464876681995543),
                from_const(0xe945e903b851b885),
                from_const(0xd794f3c95d9aa257),
                from_const(0x0d446645d8d1bc68),
                from_const(0x8d5aa6d3a3d4e158),
            ]
        );
        assert_eq!(
            hash_t16_0(
                DST,
                [
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
                ]
            ),
            from_const(0x47af4b6d88d07ed1)
        );
    }

    #[test]
    fn test_hash_t20_1() {
        assert_eq!(
            hash_t20(DST, [from_const(42),]),
            [
                from_const(0x4306c96f219fb340),
                from_const(0x61ce1a693d3f14d0),
                from_const(0xc11834730ea001c9),
                from_const(0x96023c752c07137c),
                from_const(0x581597e0f9faa28d),
                from_const(0x7dbbd1ea94384ced),
                from_const(0x7a88e951a19f61c5),
                from_const(0x06d97863b871b14d),
                from_const(0x97bea12cff2211b9),
                from_const(0xe50c051da9604a87),
                from_const(0xb60cafe5a65ae4ba),
                from_const(0xc615f84a810ce4d0),
                from_const(0x7e42d4b1a0b3ef4c),
                from_const(0x2d606bce8ccce338),
                from_const(0x1d56c84c2cb927af),
                from_const(0xe05dfef6d252b171),
            ]
        );
        assert_eq!(
            hash_t20_0(DST, [from_const(42),]),
            from_const(0x4306c96f219fb340)
        );
    }

    #[test]
    fn test_hash_t20_2() {
        assert_eq!(
            hash_t20(DST, [from_const(12), from_const(34),]),
            [
                from_const(0x57170c401eca6fdb),
                from_const(0xdbcce6820fe1cf71),
                from_const(0x28c60dee25b6330d),
                from_const(0x961c268b99502ae4),
                from_const(0x8c620b074b86d732),
                from_const(0x643d552634d7730c),
                from_const(0x4d9f004deab4458a),
                from_const(0x6e4ecfdd3f0eacbb),
                from_const(0xbdcb060144a45c85),
                from_const(0xdbf138b0a86d024d),
                from_const(0x59ca31c2002fac11),
                from_const(0x2423cf343124187b),
                from_const(0xc50e1bb90e5ddce3),
                from_const(0x30d67ee42b36037e),
                from_const(0xbc7d543c3a304c4c),
                from_const(0xd4bacb3f89fb023a),
            ]
        );
        assert_eq!(
            hash_t20_0(DST, [from_const(12), from_const(34),]),
            from_const(0x57170c401eca6fdb)
        );
    }

    #[test]
    fn test_hash_t20_16() {
        assert_eq!(
            hash_t20(
                DST,
                [
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
                ]
            ),
            [
                from_const(0xe9bc8f5856d8020f),
                from_const(0x2926b3f41dbc0904),
                from_const(0xf3865d23d15dbffa),
                from_const(0x3411a90a6c3ffcc7),
                from_const(0x693e7dcc81462529),
                from_const(0x0b66e3b30014f3a5),
                from_const(0x63bc2540ceb6e68d),
                from_const(0x540b66853c62c9e8),
                from_const(0xa4f9728000273c70),
                from_const(0xc57933c69abbf5a9),
                from_const(0x9ab6fdb2d805bd59),
                from_const(0x7a6eed835545aaf2),
                from_const(0x74d9df4b6f397c63),
                from_const(0xbd7ff160d047e569),
                from_const(0x8e59c41861615d1e),
                from_const(0x7d26431a8b73f106),
            ]
        );
        assert_eq!(
            hash_t20_0(
                DST,
                [
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
                ]
            ),
            from_const(0xe9bc8f5856d8020f)
        );
    }

    #[test]
    fn test_hash_t20_17() {
        assert_eq!(
            hash_t20(
                DST,
                [
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
                ]
            ),
            [
                from_const(0x0c0c89e5d99dde55),
                from_const(0xfb63d03172182dbb),
                from_const(0xec99a156eef77038),
                from_const(0xd5118deef72e204c),
                from_const(0xcd6c56c7477c7570),
                from_const(0x3bce7c6233435873),
                from_const(0xc5e86ffc65126d11),
                from_const(0x31fa013c96e51a07),
                from_const(0x17af574b6cb7ebdc),
                from_const(0x864cc78596f9c9c8),
                from_const(0xe8ac00692e9323ed),
                from_const(0x69b9e25d2d876237),
                from_const(0xa4a3efa01f6da959),
                from_const(0xb6af43d128f3a0be),
                from_const(0x603b4417454dc4c0),
                from_const(0x315510d0a76982f8),
            ]
        );
        assert_eq!(
            hash_t20_0(
                DST,
                [
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
                ]
            ),
            from_const(0x0c0c89e5d99dde55)
        );
    }

    #[test]
    fn test_hash_t20_19() {
        assert_eq!(
            hash_t20(
                DST,
                [
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
                    from_const(17),
                    from_const(18),
                ]
            ),
            [
                from_const(0x3525e331b2a1d601),
                from_const(0x7bf980252ca12e3c),
                from_const(0xc4ce3294929ac844),
                from_const(0x455a4f853ff7633c),
                from_const(0x0179693cd74bd118),
                from_const(0x662c9925b678199a),
                from_const(0x7b849225c66671e3),
                from_const(0x9f4901e4708d81f1),
                from_const(0x6e1671750bf0d8c2),
                from_const(0x46a3b0417d8d3e79),
                from_const(0x474104405925b061),
                from_const(0x2186cd5f7e024a2f),
                from_const(0x86c44d0aa24381ea),
                from_const(0x37f990d76c54d823),
                from_const(0x9812d015a710f385),
                from_const(0x73d1ab35c64220cc),
            ]
        );
        assert_eq!(
            hash_t20_0(
                DST,
                [
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
                    from_const(17),
                    from_const(18),
                ]
            ),
            from_const(0x3525e331b2a1d601)
        );
    }

    #[test]
    fn test_hash_t20_20() {
        assert_eq!(
            hash_t20(
                DST,
                [
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
                    from_const(17),
                    from_const(18),
                    from_const(19),
                ]
            ),
            [
                from_const(0xc3b44b70ef821010),
                from_const(0x1a255be00afbb9e4),
                from_const(0xec70f5b1e605c8e4),
                from_const(0xbbb4a81dfd1182c0),
                from_const(0x04fe6ed83d75dfff),
                from_const(0xbaadb5a73fa3ebf1),
                from_const(0x5e494632b95d1e50),
                from_const(0x9e73b3e17b0abf35),
                from_const(0xe21376e1e050f228),
                from_const(0xb46f7fc958514143),
                from_const(0xa2065c7977bd1b51),
                from_const(0xd28186c822bffaaf),
                from_const(0x48efaa06cb279210),
                from_const(0x611895aedc4a5197),
                from_const(0x546878e014fd357a),
                from_const(0x4a13610b629b11a1),
            ]
        );
        assert_eq!(
            hash_t20_0(
                DST,
                [
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
                    from_const(17),
                    from_const(18),
                    from_const(19),
                ]
            ),
            from_const(0xc3b44b70ef821010)
        );
    }

    #[test]
    fn test_hash_t20_21() {
        assert_eq!(
            hash_t20(
                DST,
                [
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
                    from_const(17),
                    from_const(18),
                    from_const(19),
                    from_const(20),
                ]
            ),
            [
                from_const(0x3b8266259e3e7449),
                from_const(0xf5f33748ccd63f3c),
                from_const(0xdc9090be36e5e71a),
                from_const(0xf891209a8c110002),
                from_const(0x1725f04c26c7631e),
                from_const(0x79c7b99fb063eadc),
                from_const(0x0240fa4bf5835491),
                from_const(0xb9a2f4e8c1ba9009),
                from_const(0x44cc401b7ff78d9a),
                from_const(0x80d201159ae51074),
                from_const(0xf06b695655e17e5a),
                from_const(0x329c1619ea3b99c1),
                from_const(0x3d282593eb171154),
                from_const(0x72c0783e4483bb30),
                from_const(0xa8533ba6334fa619),
                from_const(0x82ea678bb33c4cf2),
            ]
        );
        assert_eq!(
            hash_t20_0(
                DST,
                [
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
                    from_const(17),
                    from_const(18),
                    from_const(19),
                    from_const(20),
                ]
            ),
            from_const(0x3b8266259e3e7449)
        );
    }
}
