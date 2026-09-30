use crate::params::decode_constants;
use crate::poseidon;
use starkom_koalabear::KB as Scalar;
use std::sync::LazyLock;

/// Poseidon configuration for the KoalaBear field.
pub struct KoalaBearConfig<const T: usize> {}

impl poseidon::Config<Scalar, 24> for KoalaBearConfig<24> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        23
    }

    fn alpha() -> usize {
        3
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 744]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/koalabear/arc_t24.bin");
            decode_constants::<Scalar, 744>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 576]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/koalabear/mds_t24.bin");
            decode_constants::<Scalar, 576>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 32> for KoalaBearConfig<32> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        31
    }

    fn alpha() -> usize {
        3
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 1248]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/koalabear/arc_t32.bin");
            decode_constants::<Scalar, 1248>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 1024]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/koalabear/mds_t32.bin");
            decode_constants::<Scalar, 1024>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 40> for KoalaBearConfig<40> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        38
    }

    fn alpha() -> usize {
        3
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 1840]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/koalabear/arc_t40.bin");
            decode_constants::<Scalar, 1840>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 1600]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/koalabear/mds_t40.bin");
            decode_constants::<Scalar, 1600>(bytes)
        });
        &*MATRIX
    }
}

/// Poseidon configuration for KoalaBear with T=24.
pub type KoalaBearConfig24 = KoalaBearConfig<24>;

/// Poseidon configuration for KoalaBear with T=32.
pub type KoalaBearConfig32 = KoalaBearConfig<32>;

/// Poseidon configuration for KoalaBear with T=40.
pub type KoalaBearConfig40 = KoalaBearConfig<40>;

#[cfg(test)]
mod tests {
    use super::*;
    use starkom_ff::Field;
    use starkom_koalabear::from_const;

    const DST: [Scalar; 8] = [Scalar::ZERO; 8];

    fn hash_t24(dst: [Scalar; 8], inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 16] {
        poseidon::hash::<KoalaBearConfig24, Scalar, 24, 16, 8>(dst, inputs)
    }

    fn hash_t24_0(dst: [Scalar; 8], inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<KoalaBearConfig24, Scalar, 24, 16, 8>(dst, inputs)
    }

    fn hash_t32(dst: [Scalar; 8], inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 24] {
        poseidon::hash::<KoalaBearConfig32, Scalar, 32, 24, 8>(dst, inputs)
    }

    fn hash_t32_0(dst: [Scalar; 8], inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<KoalaBearConfig32, Scalar, 32, 24, 8>(dst, inputs)
    }

    fn hash_t40(dst: [Scalar; 8], inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 32] {
        poseidon::hash::<KoalaBearConfig40, Scalar, 40, 32, 8>(dst, inputs)
    }

    fn hash_t40_0(dst: [Scalar; 8], inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<KoalaBearConfig40, Scalar, 40, 32, 8>(dst, inputs)
    }

    fn sequential_inputs<const N: usize>() -> [Scalar; N] {
        std::array::from_fn(|i| from_const(i as u32))
    }

    #[test]
    fn test_permutation_t24() {
        assert_eq!(
            poseidon::permutation::<KoalaBearConfig24, Scalar, 24>([
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
                from_const(21),
                from_const(22),
                from_const(23),
            ]),
            [
                from_const(0x20c420b9),
                from_const(0x13ee7831),
                from_const(0x1664a5fe),
                from_const(0x61b8a9da),
                from_const(0x62c0a36b),
                from_const(0x6686aa16),
                from_const(0x086b196f),
                from_const(0x2ba9a06e),
                from_const(0x48e393fa),
                from_const(0x40e4aa30),
                from_const(0x5afc6000),
                from_const(0x5c9bd903),
                from_const(0x3d866242),
                from_const(0x44fb43ec),
                from_const(0x4fd48650),
                from_const(0x45c361f2),
                from_const(0x5087cccd),
                from_const(0x0c28329f),
                from_const(0x58fc4353),
                from_const(0x1fa8695a),
                from_const(0x4a026a9d),
                from_const(0x3e0c59ec),
                from_const(0x3be00b5d),
                from_const(0x687cd26e),
            ]
        );
    }

    #[test]
    fn test_permutation_t32() {
        assert_eq!(
            poseidon::permutation::<KoalaBearConfig32, Scalar, 32>([
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
                from_const(21),
                from_const(22),
                from_const(23),
                from_const(24),
                from_const(25),
                from_const(26),
                from_const(27),
                from_const(28),
                from_const(29),
                from_const(30),
                from_const(31),
            ]),
            [
                from_const(0x2cd28abd),
                from_const(0x4b7e0de6),
                from_const(0x1c28010b),
                from_const(0x341a4215),
                from_const(0x2730f966),
                from_const(0x6d5f6492),
                from_const(0x2da41e53),
                from_const(0x5b9553f8),
                from_const(0x40f3bd78),
                from_const(0x1847d418),
                from_const(0x3b40f194),
                from_const(0x61deec2d),
                from_const(0x268de447),
                from_const(0x4ff70519),
                from_const(0x0276d981),
                from_const(0x0ecd4498),
                from_const(0x5852c99b),
                from_const(0x0c0c5656),
                from_const(0x1a5d72b5),
                from_const(0x1556aa75),
                from_const(0x05df8f05),
                from_const(0x4e9c5eb7),
                from_const(0x669108c9),
                from_const(0x051bc5f5),
                from_const(0x5d193fdf),
                from_const(0x1fc5ddac),
                from_const(0x478e26aa),
                from_const(0x78fe0af3),
                from_const(0x40b6dd16),
                from_const(0x6cc4f791),
                from_const(0x77e872fe),
                from_const(0x6112c017),
            ]
        );
    }

    #[test]
    fn test_permutation_t40() {
        assert_eq!(
            poseidon::permutation::<KoalaBearConfig40, Scalar, 40>(sequential_inputs::<40>()),
            [
                from_const(0x1885f836),
                from_const(0x07698f9c),
                from_const(0x12fe7371),
                from_const(0x0b522da1),
                from_const(0x763dd418),
                from_const(0x7caf7bfb),
                from_const(0x09459508),
                from_const(0x03aadccb),
                from_const(0x22430bc5),
                from_const(0x2d934a4d),
                from_const(0x083c939f),
                from_const(0x01cbb222),
                from_const(0x6e273814),
                from_const(0x18319f5e),
                from_const(0x61c30523),
                from_const(0x596f1085),
                from_const(0x325b24ba),
                from_const(0x70ac050e),
                from_const(0x007c2dd0),
                from_const(0x7ef0794a),
                from_const(0x2a9a0cce),
                from_const(0x2c7e9e22),
                from_const(0x3611b8c7),
                from_const(0x40842ab8),
                from_const(0x0ba6a8b0),
                from_const(0x228febc4),
                from_const(0x5b8045bc),
                from_const(0x603a133f),
                from_const(0x2c31e2bb),
                from_const(0x055c27c4),
                from_const(0x664c03b8),
                from_const(0x2037b3ee),
                from_const(0x55713355),
                from_const(0x0cde8292),
                from_const(0x5b2dd2fb),
                from_const(0x3be682a6),
                from_const(0x1ef9ec00),
                from_const(0x327d8eaa),
                from_const(0x3a23d724),
                from_const(0x7be639ea),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t24() {
        assert_eq!(
            hash_t24(
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
                from_const(0x3cbc6d5c),
                from_const(0x3efc8c2b),
                from_const(0x4c2b7b2c),
                from_const(0x4996b827),
                from_const(0x6d7b064c),
                from_const(0x1fe33565),
                from_const(0x6886a23b),
                from_const(0x6ecb7a72),
                from_const(0x3a3cf295),
                from_const(0x11f070e0),
                from_const(0x3b464b06),
                from_const(0x237378d9),
                from_const(0x1ea544eb),
                from_const(0x5f12ea4a),
                from_const(0x255ad713),
                from_const(0x57b6f3df),
            ]
        );
        assert_eq!(
            hash_t24(
                [
                    from_const(16),
                    from_const(17),
                    from_const(18),
                    from_const(19),
                    from_const(20),
                    from_const(21),
                    from_const(22),
                    from_const(23),
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
                from_const(0x20c420b9),
                from_const(0x13ee7831),
                from_const(0x1664a5fe),
                from_const(0x61b8a9da),
                from_const(0x62c0a36b),
                from_const(0x6686aa16),
                from_const(0x086b196f),
                from_const(0x2ba9a06e),
                from_const(0x48e393fa),
                from_const(0x40e4aa30),
                from_const(0x5afc6000),
                from_const(0x5c9bd903),
                from_const(0x3d866242),
                from_const(0x44fb43ec),
                from_const(0x4fd48650),
                from_const(0x45c361f2),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t32() {
        assert_eq!(
            hash_t32(
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
                    from_const(21),
                    from_const(22),
                    from_const(23),
                ]
            ),
            [
                from_const(0x2669e81d),
                from_const(0x37275f95),
                from_const(0x334ed52d),
                from_const(0x0da08077),
                from_const(0x546df638),
                from_const(0x0e142ef5),
                from_const(0x2554cf75),
                from_const(0x7aeabbcf),
                from_const(0x16daecae),
                from_const(0x01aadbe8),
                from_const(0x4623b19a),
                from_const(0x607faa1b),
                from_const(0x108e93f7),
                from_const(0x7e1153a0),
                from_const(0x450b79c8),
                from_const(0x15d0bdc9),
                from_const(0x487ec2bb),
                from_const(0x21f8dc0e),
                from_const(0x14f9f357),
                from_const(0x657ad505),
                from_const(0x15f440b7),
                from_const(0x27389ded),
                from_const(0x5ec33eba),
                from_const(0x77ae7dca),
            ]
        );
        assert_eq!(
            hash_t32(
                [
                    from_const(24),
                    from_const(25),
                    from_const(26),
                    from_const(27),
                    from_const(28),
                    from_const(29),
                    from_const(30),
                    from_const(31),
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
                    from_const(16),
                    from_const(17),
                    from_const(18),
                    from_const(19),
                    from_const(20),
                    from_const(21),
                    from_const(22),
                    from_const(23),
                ]
            ),
            [
                from_const(0x2cd28abd),
                from_const(0x4b7e0de6),
                from_const(0x1c28010b),
                from_const(0x341a4215),
                from_const(0x2730f966),
                from_const(0x6d5f6492),
                from_const(0x2da41e53),
                from_const(0x5b9553f8),
                from_const(0x40f3bd78),
                from_const(0x1847d418),
                from_const(0x3b40f194),
                from_const(0x61deec2d),
                from_const(0x268de447),
                from_const(0x4ff70519),
                from_const(0x0276d981),
                from_const(0x0ecd4498),
                from_const(0x5852c99b),
                from_const(0x0c0c5656),
                from_const(0x1a5d72b5),
                from_const(0x1556aa75),
                from_const(0x05df8f05),
                from_const(0x4e9c5eb7),
                from_const(0x669108c9),
                from_const(0x051bc5f5),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t40() {
        assert_eq!(
            hash_t40(DST, sequential_inputs::<32>()),
            [
                from_const(0x45cb2dac),
                from_const(0x66bf4fe1),
                from_const(0x40090717),
                from_const(0x3c7ac696),
                from_const(0x58759d40),
                from_const(0x02d418f3),
                from_const(0x17b80402),
                from_const(0x72a50968),
                from_const(0x4d34902a),
                from_const(0x27b35fd9),
                from_const(0x2354ae8d),
                from_const(0x2ccd2a5d),
                from_const(0x232c5eda),
                from_const(0x7c2061fd),
                from_const(0x21c3ecd1),
                from_const(0x6f751e38),
                from_const(0x3c150766),
                from_const(0x6dc7623a),
                from_const(0x30a564aa),
                from_const(0x19c23b9f),
                from_const(0x360f77ba),
                from_const(0x67869e71),
                from_const(0x326b886b),
                from_const(0x24fbe771),
                from_const(0x26a1acae),
                from_const(0x3b7f9e84),
                from_const(0x35991525),
                from_const(0x2f5089a0),
                from_const(0x6481c48d),
                from_const(0x518a1a28),
                from_const(0x072760d4),
                from_const(0x5c097cf4),
            ]
        );
        assert_eq!(
            hash_t40(
                std::array::from_fn(|i| from_const(32 + i as u32)),
                sequential_inputs::<32>()
            ),
            [
                from_const(0x1885f836),
                from_const(0x07698f9c),
                from_const(0x12fe7371),
                from_const(0x0b522da1),
                from_const(0x763dd418),
                from_const(0x7caf7bfb),
                from_const(0x09459508),
                from_const(0x03aadccb),
                from_const(0x22430bc5),
                from_const(0x2d934a4d),
                from_const(0x083c939f),
                from_const(0x01cbb222),
                from_const(0x6e273814),
                from_const(0x18319f5e),
                from_const(0x61c30523),
                from_const(0x596f1085),
                from_const(0x325b24ba),
                from_const(0x70ac050e),
                from_const(0x007c2dd0),
                from_const(0x7ef0794a),
                from_const(0x2a9a0cce),
                from_const(0x2c7e9e22),
                from_const(0x3611b8c7),
                from_const(0x40842ab8),
                from_const(0x0ba6a8b0),
                from_const(0x228febc4),
                from_const(0x5b8045bc),
                from_const(0x603a133f),
                from_const(0x2c31e2bb),
                from_const(0x055c27c4),
                from_const(0x664c03b8),
                from_const(0x2037b3ee),
            ]
        );
    }

    #[test]
    fn test_hash_t24_1() {
        assert_eq!(
            hash_t24(DST, [from_const(42)]),
            [
                from_const(0x16203a99),
                from_const(0x675ad6f6),
                from_const(0x72fc45e1),
                from_const(0x39dc9595),
                from_const(0x11875376),
                from_const(0x7cc6069c),
                from_const(0x15b57f11),
                from_const(0x408936e8),
                from_const(0x3d45d792),
                from_const(0x541b3686),
                from_const(0x509a4eaa),
                from_const(0x5da73d2c),
                from_const(0x0080016c),
                from_const(0x1cb0a94a),
                from_const(0x6dea097c),
                from_const(0x074a0511),
            ]
        );
        assert_eq!(hash_t24_0(DST, [from_const(42)]), from_const(0x16203a99));
    }

    #[test]
    fn test_hash_t24_2() {
        assert_eq!(
            hash_t24(DST, [from_const(12), from_const(34)]),
            [
                from_const(0x682ff547),
                from_const(0x68012e80),
                from_const(0x3d02790d),
                from_const(0x668170a9),
                from_const(0x639e0cff),
                from_const(0x02064fca),
                from_const(0x2be6145e),
                from_const(0x27c846ed),
                from_const(0x6d88bcda),
                from_const(0x08ba3e90),
                from_const(0x5c6f780e),
                from_const(0x2c0471c7),
                from_const(0x3d560672),
                from_const(0x03dc2b92),
                from_const(0x64a6c1bb),
                from_const(0x083e63ae),
            ]
        );
        assert_eq!(
            hash_t24_0(DST, [from_const(12), from_const(34)]),
            from_const(0x682ff547)
        );
    }

    #[test]
    fn test_hash_t24_16() {
        assert_eq!(
            hash_t24(DST, sequential_inputs::<16>()),
            [
                from_const(0x3cbc6d5c),
                from_const(0x3efc8c2b),
                from_const(0x4c2b7b2c),
                from_const(0x4996b827),
                from_const(0x6d7b064c),
                from_const(0x1fe33565),
                from_const(0x6886a23b),
                from_const(0x6ecb7a72),
                from_const(0x3a3cf295),
                from_const(0x11f070e0),
                from_const(0x3b464b06),
                from_const(0x237378d9),
                from_const(0x1ea544eb),
                from_const(0x5f12ea4a),
                from_const(0x255ad713),
                from_const(0x57b6f3df),
            ]
        );
        assert_eq!(
            hash_t24_0(DST, sequential_inputs::<16>()),
            from_const(0x3cbc6d5c)
        );
    }

    #[test]
    fn test_hash_t24_17() {
        assert_eq!(
            hash_t24(DST, sequential_inputs::<17>()),
            [
                from_const(0x12db4aa5),
                from_const(0x0c3496d0),
                from_const(0x73458a62),
                from_const(0x10d47f48),
                from_const(0x008e13c1),
                from_const(0x759827cc),
                from_const(0x42680be9),
                from_const(0x3fc2112c),
                from_const(0x35d20a10),
                from_const(0x66b6c1a8),
                from_const(0x747381f8),
                from_const(0x6ecb26a6),
                from_const(0x71b7a5b6),
                from_const(0x5bbd3a9e),
                from_const(0x387ce5a4),
                from_const(0x16b48943),
            ]
        );
        assert_eq!(
            hash_t24_0(DST, sequential_inputs::<17>()),
            from_const(0x12db4aa5)
        );
    }

    #[test]
    fn test_hash_t24_23() {
        assert_eq!(
            hash_t24(DST, sequential_inputs::<23>()),
            [
                from_const(0x6ff7c2fa),
                from_const(0x1c0d103c),
                from_const(0x2aef16be),
                from_const(0x03721307),
                from_const(0x43e01e85),
                from_const(0x3ea6243f),
                from_const(0x0397204a),
                from_const(0x172e60e2),
                from_const(0x43a8154e),
                from_const(0x7b4dc8d0),
                from_const(0x4af0974f),
                from_const(0x69036861),
                from_const(0x3c8be546),
                from_const(0x3c7fabf7),
                from_const(0x40ad7f48),
                from_const(0x634bacda),
            ]
        );
        assert_eq!(
            hash_t24_0(DST, sequential_inputs::<23>()),
            from_const(0x6ff7c2fa)
        );
    }

    #[test]
    fn test_hash_t24_24() {
        assert_eq!(
            hash_t24(DST, sequential_inputs::<24>()),
            [
                from_const(0x617265ca),
                from_const(0x48452b65),
                from_const(0x7bcbebbc),
                from_const(0x1e757dfa),
                from_const(0x13ab75e4),
                from_const(0x7737ed87),
                from_const(0x15489104),
                from_const(0x3d373bc2),
                from_const(0x349fe5c6),
                from_const(0x7e28163f),
                from_const(0x53af722c),
                from_const(0x6015abd3),
                from_const(0x6e495b22),
                from_const(0x3a52372c),
                from_const(0x400167da),
                from_const(0x674c02db),
            ]
        );
        assert_eq!(
            hash_t24_0(DST, sequential_inputs::<24>()),
            from_const(0x617265ca)
        );
    }

    #[test]
    fn test_hash_t24_25() {
        assert_eq!(
            hash_t24(DST, sequential_inputs::<25>()),
            [
                from_const(0x27492ed2),
                from_const(0x4bfd627d),
                from_const(0x07081f84),
                from_const(0x5551716c),
                from_const(0x606a4ad5),
                from_const(0x0d3b0acf),
                from_const(0x36cbdd7c),
                from_const(0x2d24dfa9),
                from_const(0x2ef2a669),
                from_const(0x2eef6827),
                from_const(0x5f484bac),
                from_const(0x1183d183),
                from_const(0x33cac7a1),
                from_const(0x2f1445c0),
                from_const(0x01da4877),
                from_const(0x2da76494),
            ]
        );
        assert_eq!(
            hash_t24_0(DST, sequential_inputs::<25>()),
            from_const(0x27492ed2)
        );
    }

    #[test]
    fn test_hash_t32_1() {
        assert_eq!(
            hash_t32(DST, [from_const(42)]),
            [
                from_const(0x124138c6),
                from_const(0x76b95d35),
                from_const(0x53035a87),
                from_const(0x367161fb),
                from_const(0x6663a034),
                from_const(0x4137c32d),
                from_const(0x18a7cedb),
                from_const(0x47b62a63),
                from_const(0x3b174224),
                from_const(0x4894746c),
                from_const(0x0004353f),
                from_const(0x21742617),
                from_const(0x55648fc8),
                from_const(0x73ab4f25),
                from_const(0x42d049ac),
                from_const(0x7d0b3bd7),
                from_const(0x7bf148a3),
                from_const(0x48369481),
                from_const(0x7469eb77),
                from_const(0x3a301ebf),
                from_const(0x1e99595f),
                from_const(0x309458ed),
                from_const(0x7bfe919e),
                from_const(0x2c6e2995),
            ]
        );
        assert_eq!(hash_t32_0(DST, [from_const(42)]), from_const(0x124138c6));
    }

    #[test]
    fn test_hash_t32_2() {
        assert_eq!(
            hash_t32(DST, [from_const(12), from_const(34)]),
            [
                from_const(0x3ca81acf),
                from_const(0x15096cdf),
                from_const(0x3d3f61f4),
                from_const(0x0315aa5a),
                from_const(0x50d7c9be),
                from_const(0x4b14c725),
                from_const(0x2261eaf8),
                from_const(0x0adc5a39),
                from_const(0x41a527a1),
                from_const(0x7a856336),
                from_const(0x669fab74),
                from_const(0x5d7ed6c6),
                from_const(0x51878b10),
                from_const(0x5137c12f),
                from_const(0x1495784f),
                from_const(0x3e3b4066),
                from_const(0x63e5ad32),
                from_const(0x2468829f),
                from_const(0x58e282b6),
                from_const(0x2e6a84e5),
                from_const(0x2ca88ba1),
                from_const(0x438bd9d7),
                from_const(0x1842a516),
                from_const(0x142a60f5),
            ]
        );
        assert_eq!(
            hash_t32_0(DST, [from_const(12), from_const(34)]),
            from_const(0x3ca81acf)
        );
    }

    #[test]
    fn test_hash_t32_24() {
        assert_eq!(
            hash_t32(DST, sequential_inputs::<24>()),
            [
                from_const(0x2669e81d),
                from_const(0x37275f95),
                from_const(0x334ed52d),
                from_const(0x0da08077),
                from_const(0x546df638),
                from_const(0x0e142ef5),
                from_const(0x2554cf75),
                from_const(0x7aeabbcf),
                from_const(0x16daecae),
                from_const(0x01aadbe8),
                from_const(0x4623b19a),
                from_const(0x607faa1b),
                from_const(0x108e93f7),
                from_const(0x7e1153a0),
                from_const(0x450b79c8),
                from_const(0x15d0bdc9),
                from_const(0x487ec2bb),
                from_const(0x21f8dc0e),
                from_const(0x14f9f357),
                from_const(0x657ad505),
                from_const(0x15f440b7),
                from_const(0x27389ded),
                from_const(0x5ec33eba),
                from_const(0x77ae7dca),
            ]
        );
        assert_eq!(
            hash_t32_0(DST, sequential_inputs::<24>()),
            from_const(0x2669e81d)
        );
    }

    #[test]
    fn test_hash_t32_25() {
        assert_eq!(
            hash_t32(DST, sequential_inputs::<25>()),
            [
                from_const(0x43300146),
                from_const(0x0806f638),
                from_const(0x688750b5),
                from_const(0x3cebd421),
                from_const(0x48f5a1ac),
                from_const(0x167b6a04),
                from_const(0x18da050c),
                from_const(0x37727f40),
                from_const(0x2e9038ec),
                from_const(0x4af1c7ad),
                from_const(0x6527807a),
                from_const(0x636018fb),
                from_const(0x45ee4954),
                from_const(0x2558227d),
                from_const(0x6238c54c),
                from_const(0x4d946b65),
                from_const(0x2ac34872),
                from_const(0x0ea840d4),
                from_const(0x3b47bded),
                from_const(0x4790cf2f),
                from_const(0x509707f8),
                from_const(0x22d5190a),
                from_const(0x34b652e2),
                from_const(0x4eaf206d),
            ]
        );
        assert_eq!(
            hash_t32_0(DST, sequential_inputs::<25>()),
            from_const(0x43300146)
        );
    }

    #[test]
    fn test_hash_t32_31() {
        assert_eq!(
            hash_t32(DST, sequential_inputs::<31>()),
            [
                from_const(0x2928a1ce),
                from_const(0x622fa59f),
                from_const(0x0a8f6152),
                from_const(0x3ccf79d0),
                from_const(0x7ab31882),
                from_const(0x65c6d7c2),
                from_const(0x2d0b4f19),
                from_const(0x5bbadc84),
                from_const(0x73638bb7),
                from_const(0x57e6dd48),
                from_const(0x7dd6040c),
                from_const(0x238fa206),
                from_const(0x34c0ade1),
                from_const(0x76d109e0),
                from_const(0x22e3b187),
                from_const(0x4619cc20),
                from_const(0x5971a5bc),
                from_const(0x444989cd),
                from_const(0x22d1dc0a),
                from_const(0x1f7fe5b1),
                from_const(0x340a36b1),
                from_const(0x102c3e83),
                from_const(0x647ed19e),
                from_const(0x5ee20ca6),
            ]
        );
        assert_eq!(
            hash_t32_0(DST, sequential_inputs::<31>()),
            from_const(0x2928a1ce)
        );
    }

    #[test]
    fn test_hash_t32_32() {
        assert_eq!(
            hash_t32(DST, sequential_inputs::<32>()),
            [
                from_const(0x33fb7eab),
                from_const(0x01effac1),
                from_const(0x274e2b70),
                from_const(0x672b815b),
                from_const(0x3aeaa84c),
                from_const(0x557d0634),
                from_const(0x196e5ff9),
                from_const(0x13971cd0),
                from_const(0x514ab220),
                from_const(0x5417c64f),
                from_const(0x0bf2d9db),
                from_const(0x5b86b2c1),
                from_const(0x20ef6d6b),
                from_const(0x34f9b91b),
                from_const(0x427c8b16),
                from_const(0x2168bc12),
                from_const(0x27e19c41),
                from_const(0x754ea2f6),
                from_const(0x7bb97f09),
                from_const(0x2cffdeb4),
                from_const(0x5b8e892d),
                from_const(0x295d8864),
                from_const(0x46e93683),
                from_const(0x27f0e4f7),
            ]
        );
        assert_eq!(
            hash_t32_0(DST, sequential_inputs::<32>()),
            from_const(0x33fb7eab)
        );
    }

    #[test]
    fn test_hash_t32_33() {
        assert_eq!(
            hash_t32(DST, sequential_inputs::<33>()),
            [
                from_const(0x41a34e14),
                from_const(0x299a4cc1),
                from_const(0x3ebdb5e4),
                from_const(0x5c300b2d),
                from_const(0x0c7ee491),
                from_const(0x0d16cb78),
                from_const(0x757cd076),
                from_const(0x0159a359),
                from_const(0x250bbb48),
                from_const(0x74d72264),
                from_const(0x74df1f73),
                from_const(0x0067602d),
                from_const(0x59b6eee8),
                from_const(0x70bf88bf),
                from_const(0x7443dc01),
                from_const(0x4e989666),
                from_const(0x0757c1ef),
                from_const(0x3a023c44),
                from_const(0x64af3cbd),
                from_const(0x610ae2b8),
                from_const(0x5350655a),
                from_const(0x42bf4bad),
                from_const(0x4f2f5c47),
                from_const(0x282b07e5),
            ]
        );
        assert_eq!(
            hash_t32_0(DST, sequential_inputs::<33>()),
            from_const(0x41a34e14)
        );
    }

    #[test]
    fn test_hash_t40_1() {
        assert_eq!(
            hash_t40(DST, [from_const(42)]),
            [
                from_const(0x1e1a0e27),
                from_const(0x75c17550),
                from_const(0x0bd455a6),
                from_const(0x353beb27),
                from_const(0x4c392bad),
                from_const(0x1c061064),
                from_const(0x591ab01f),
                from_const(0x51d1b053),
                from_const(0x4f6bb750),
                from_const(0x39185c7f),
                from_const(0x31a59293),
                from_const(0x25e89ba9),
                from_const(0x447416ab),
                from_const(0x0d68e1c4),
                from_const(0x2a4a894b),
                from_const(0x1d335201),
                from_const(0x0febc6b6),
                from_const(0x569fe3cb),
                from_const(0x75c6bbc3),
                from_const(0x0c6088c7),
                from_const(0x56cb597b),
                from_const(0x519804b6),
                from_const(0x31ca978a),
                from_const(0x24a9ee94),
                from_const(0x68442ed6),
                from_const(0x0bee2618),
                from_const(0x167e7879),
                from_const(0x0bdb3375),
                from_const(0x17817a29),
                from_const(0x7970d9a6),
                from_const(0x6c3eac12),
                from_const(0x35734882),
            ]
        );
        assert_eq!(hash_t40_0(DST, [from_const(42)]), from_const(0x1e1a0e27));
    }

    #[test]
    fn test_hash_t40_2() {
        assert_eq!(
            hash_t40(DST, [from_const(12), from_const(34)]),
            [
                from_const(0x5feab079),
                from_const(0x1c1798e0),
                from_const(0x0b8a4bd2),
                from_const(0x595ea079),
                from_const(0x5903f873),
                from_const(0x4a3668b3),
                from_const(0x1bf83492),
                from_const(0x244e35bd),
                from_const(0x2b823be5),
                from_const(0x60789b19),
                from_const(0x29012b2e),
                from_const(0x07e9f565),
                from_const(0x78da8572),
                from_const(0x616d5154),
                from_const(0x7bf7798a),
                from_const(0x2fbc6ff6),
                from_const(0x4e629021),
                from_const(0x338082ee),
                from_const(0x6e4d26eb),
                from_const(0x78bd8f95),
                from_const(0x5bf74bc0),
                from_const(0x1752a894),
                from_const(0x02b097d3),
                from_const(0x5d80be9a),
                from_const(0x60dea498),
                from_const(0x5a2e2ee6),
                from_const(0x15e573c1),
                from_const(0x6155b781),
                from_const(0x3207f4a0),
                from_const(0x40fc9976),
                from_const(0x15ad8442),
                from_const(0x3a75107b),
            ]
        );
        assert_eq!(
            hash_t40_0(DST, [from_const(12), from_const(34)]),
            from_const(0x5feab079)
        );
    }

    #[test]
    fn test_hash_t40_32() {
        assert_eq!(
            hash_t40(DST, sequential_inputs::<32>()),
            [
                from_const(0x45cb2dac),
                from_const(0x66bf4fe1),
                from_const(0x40090717),
                from_const(0x3c7ac696),
                from_const(0x58759d40),
                from_const(0x02d418f3),
                from_const(0x17b80402),
                from_const(0x72a50968),
                from_const(0x4d34902a),
                from_const(0x27b35fd9),
                from_const(0x2354ae8d),
                from_const(0x2ccd2a5d),
                from_const(0x232c5eda),
                from_const(0x7c2061fd),
                from_const(0x21c3ecd1),
                from_const(0x6f751e38),
                from_const(0x3c150766),
                from_const(0x6dc7623a),
                from_const(0x30a564aa),
                from_const(0x19c23b9f),
                from_const(0x360f77ba),
                from_const(0x67869e71),
                from_const(0x326b886b),
                from_const(0x24fbe771),
                from_const(0x26a1acae),
                from_const(0x3b7f9e84),
                from_const(0x35991525),
                from_const(0x2f5089a0),
                from_const(0x6481c48d),
                from_const(0x518a1a28),
                from_const(0x072760d4),
                from_const(0x5c097cf4),
            ]
        );
        assert_eq!(
            hash_t40_0(DST, sequential_inputs::<32>()),
            from_const(0x45cb2dac)
        );
    }

    #[test]
    fn test_hash_t40_33() {
        assert_eq!(
            hash_t40(DST, sequential_inputs::<33>()),
            [
                from_const(0x44a49d74),
                from_const(0x7ea95a12),
                from_const(0x0756eaee),
                from_const(0x1d274118),
                from_const(0x2b1a875c),
                from_const(0x02082221),
                from_const(0x115b6637),
                from_const(0x4a38061d),
                from_const(0x2554fba4),
                from_const(0x6679c343),
                from_const(0x5b47846b),
                from_const(0x7ebce96d),
                from_const(0x6f95ecb1),
                from_const(0x78e4da32),
                from_const(0x753f1552),
                from_const(0x1c2d8fa5),
                from_const(0x4fa73c59),
                from_const(0x78a44712),
                from_const(0x3d6fa255),
                from_const(0x76fe02ca),
                from_const(0x6dbdfd60),
                from_const(0x7e3e8e28),
                from_const(0x0b949825),
                from_const(0x07edf50c),
                from_const(0x6149a3ca),
                from_const(0x3280a266),
                from_const(0x2b311889),
                from_const(0x2682c6af),
                from_const(0x7295aec9),
                from_const(0x3b8c3b5f),
                from_const(0x0f0e000c),
                from_const(0x0b6384a0),
            ]
        );
        assert_eq!(
            hash_t40_0(DST, sequential_inputs::<33>()),
            from_const(0x44a49d74)
        );
    }

    #[test]
    fn test_hash_t40_39() {
        assert_eq!(
            hash_t40(DST, sequential_inputs::<39>()),
            [
                from_const(0x1d41c30c),
                from_const(0x6c116182),
                from_const(0x57fa26fa),
                from_const(0x515a5b33),
                from_const(0x1406ba88),
                from_const(0x0320fc2d),
                from_const(0x56871139),
                from_const(0x6579edad),
                from_const(0x517440a4),
                from_const(0x7ef669c8),
                from_const(0x50274f76),
                from_const(0x092408ce),
                from_const(0x2fd45dbd),
                from_const(0x683c34b8),
                from_const(0x6f4f735e),
                from_const(0x6c1d23ff),
                from_const(0x17f05cb2),
                from_const(0x0941df7e),
                from_const(0x72b513ee),
                from_const(0x4bbee87e),
                from_const(0x21858131),
                from_const(0x704ce007),
                from_const(0x26e943cc),
                from_const(0x6f47bbde),
                from_const(0x38514267),
                from_const(0x0d4fff69),
                from_const(0x4f583d47),
                from_const(0x102404b3),
                from_const(0x3ec1965e),
                from_const(0x0c3afca7),
                from_const(0x62088740),
                from_const(0x5c36fa57),
            ]
        );
        assert_eq!(
            hash_t40_0(DST, sequential_inputs::<39>()),
            from_const(0x1d41c30c)
        );
    }

    #[test]
    fn test_hash_t40_40() {
        assert_eq!(
            hash_t40(DST, sequential_inputs::<40>()),
            [
                from_const(0x17c85b48),
                from_const(0x08e373f7),
                from_const(0x0171e923),
                from_const(0x14a6760f),
                from_const(0x66090240),
                from_const(0x79be3bf4),
                from_const(0x3d9a718b),
                from_const(0x2703358d),
                from_const(0x54df6975),
                from_const(0x2083c41c),
                from_const(0x1ec35dc1),
                from_const(0x3bcd1047),
                from_const(0x04a9956a),
                from_const(0x19af1658),
                from_const(0x3d4ceadd),
                from_const(0x1b6c7f0d),
                from_const(0x370aa7f9),
                from_const(0x075d8f81),
                from_const(0x5725a3f2),
                from_const(0x5794c1a8),
                from_const(0x1ec59d11),
                from_const(0x356480a8),
                from_const(0x12a7c42e),
                from_const(0x5d029393),
                from_const(0x132b0940),
                from_const(0x7d3e51ea),
                from_const(0x3e76c7ee),
                from_const(0x38c85e1b),
                from_const(0x50828377),
                from_const(0x0daf2100),
                from_const(0x4b550c0e),
                from_const(0x7de728a3),
            ]
        );
        assert_eq!(
            hash_t40_0(DST, sequential_inputs::<40>()),
            from_const(0x17c85b48)
        );
    }

    #[test]
    fn test_hash_t40_41() {
        assert_eq!(
            hash_t40(DST, sequential_inputs::<41>()),
            [
                from_const(0x1537e956),
                from_const(0x674a98b1),
                from_const(0x33be5663),
                from_const(0x18f8eb17),
                from_const(0x58cf90ff),
                from_const(0x4962d58c),
                from_const(0x5ce480f1),
                from_const(0x61114fc5),
                from_const(0x42decc3c),
                from_const(0x3e9c82c1),
                from_const(0x569ff6f6),
                from_const(0x2258ff5c),
                from_const(0x281f3807),
                from_const(0x4663362c),
                from_const(0x0848bea5),
                from_const(0x243482cb),
                from_const(0x4bf6a79c),
                from_const(0x61034cb1),
                from_const(0x0faab563),
                from_const(0x2140798a),
                from_const(0x6baf9fbd),
                from_const(0x0de6aa20),
                from_const(0x215e7e20),
                from_const(0x59fdb3ee),
                from_const(0x5e0b523c),
                from_const(0x66dfad4f),
                from_const(0x0e321630),
                from_const(0x0d602fab),
                from_const(0x0364863d),
                from_const(0x5262dad9),
                from_const(0x55fb41a9),
                from_const(0x24bdc933),
            ]
        );
        assert_eq!(
            hash_t40_0(DST, sequential_inputs::<41>()),
            from_const(0x1537e956)
        );
    }
}
