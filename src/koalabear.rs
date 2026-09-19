use crate::params::decode_constants;
use crate::poseidon;
use starkom_koalabear::KB as Scalar;
use std::sync::LazyLock;

/// Poseidon configuration for the KoalaBear field.
pub struct KoalaBearConfig<const T: usize> {}

impl poseidon::Config<Scalar, 24> for KoalaBearConfig<24> {
    fn num_full_rounds() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        23
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
    fn num_full_rounds() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        31
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

/// Poseidon configuration for KoalaBear with T=24.
pub type KoalaBearConfig24 = KoalaBearConfig<24>;

/// Poseidon configuration for KoalaBear with T=32.
pub type KoalaBearConfig32 = KoalaBearConfig<32>;

#[cfg(test)]
mod tests {
    use super::*;
    use starkom_koalabear::from_const;

    fn hash_t24(inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 16] {
        poseidon::hash::<KoalaBearConfig24, Scalar, 24, 16, 8>(inputs)
    }

    fn hash_t24_0(inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<KoalaBearConfig24, Scalar, 24, 16, 8>(inputs)
    }

    fn hash_t32(inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 24] {
        poseidon::hash::<KoalaBearConfig32, Scalar, 32, 24, 8>(inputs)
    }

    fn hash_t32_0(inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<KoalaBearConfig32, Scalar, 32, 24, 8>(inputs)
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
    fn test_hash_t24_1() {
        assert_eq!(
            hash_t24([from_const(42)]),
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
        assert_eq!(hash_t24_0([from_const(42)]), from_const(0x16203a99));
    }

    #[test]
    fn test_hash_t24_2() {
        assert_eq!(
            hash_t24([from_const(12), from_const(34)]),
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
            hash_t24_0([from_const(12), from_const(34)]),
            from_const(0x682ff547)
        );
    }

    #[test]
    fn test_hash_t24_16() {
        assert_eq!(
            hash_t24(sequential_inputs::<16>()),
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
            hash_t24_0(sequential_inputs::<16>()),
            from_const(0x3cbc6d5c)
        );
    }

    #[test]
    fn test_hash_t24_17() {
        assert_eq!(
            hash_t24(sequential_inputs::<17>()),
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
            hash_t24_0(sequential_inputs::<17>()),
            from_const(0x12db4aa5)
        );
    }

    #[test]
    fn test_hash_t24_23() {
        assert_eq!(
            hash_t24(sequential_inputs::<23>()),
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
            hash_t24_0(sequential_inputs::<23>()),
            from_const(0x6ff7c2fa)
        );
    }

    #[test]
    fn test_hash_t24_24() {
        assert_eq!(
            hash_t24(sequential_inputs::<24>()),
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
            hash_t24_0(sequential_inputs::<24>()),
            from_const(0x617265ca)
        );
    }

    #[test]
    fn test_hash_t24_25() {
        assert_eq!(
            hash_t24(sequential_inputs::<25>()),
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
            hash_t24_0(sequential_inputs::<25>()),
            from_const(0x27492ed2)
        );
    }

    #[test]
    fn test_hash_t32_1() {
        assert_eq!(
            hash_t32([from_const(42)]),
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
        assert_eq!(hash_t32_0([from_const(42)]), from_const(0x124138c6));
    }

    #[test]
    fn test_hash_t32_2() {
        assert_eq!(
            hash_t32([from_const(12), from_const(34)]),
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
            hash_t32_0([from_const(12), from_const(34)]),
            from_const(0x3ca81acf)
        );
    }

    #[test]
    fn test_hash_t32_24() {
        assert_eq!(
            hash_t32(sequential_inputs::<24>()),
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
            hash_t32_0(sequential_inputs::<24>()),
            from_const(0x2669e81d)
        );
    }

    #[test]
    fn test_hash_t32_25() {
        assert_eq!(
            hash_t32(sequential_inputs::<25>()),
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
            hash_t32_0(sequential_inputs::<25>()),
            from_const(0x43300146)
        );
    }

    #[test]
    fn test_hash_t32_31() {
        assert_eq!(
            hash_t32(sequential_inputs::<31>()),
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
            hash_t32_0(sequential_inputs::<31>()),
            from_const(0x2928a1ce)
        );
    }

    #[test]
    fn test_hash_t32_32() {
        assert_eq!(
            hash_t32(sequential_inputs::<32>()),
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
            hash_t32_0(sequential_inputs::<32>()),
            from_const(0x33fb7eab)
        );
    }

    #[test]
    fn test_hash_t32_33() {
        assert_eq!(
            hash_t32(sequential_inputs::<33>()),
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
            hash_t32_0(sequential_inputs::<33>()),
            from_const(0x41a34e14)
        );
    }
}
