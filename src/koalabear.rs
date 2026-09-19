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

    fn parse_scalar(s: &'static str) -> Scalar {
        s.parse().unwrap()
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
                parse_scalar("0x20c420b9"),
                parse_scalar("0x13ee7831"),
                parse_scalar("0x1664a5fe"),
                parse_scalar("0x61b8a9da"),
                parse_scalar("0x62c0a36b"),
                parse_scalar("0x6686aa16"),
                parse_scalar("0x086b196f"),
                parse_scalar("0x2ba9a06e"),
                parse_scalar("0x48e393fa"),
                parse_scalar("0x40e4aa30"),
                parse_scalar("0x5afc6000"),
                parse_scalar("0x5c9bd903"),
                parse_scalar("0x3d866242"),
                parse_scalar("0x44fb43ec"),
                parse_scalar("0x4fd48650"),
                parse_scalar("0x45c361f2"),
                parse_scalar("0x5087cccd"),
                parse_scalar("0x0c28329f"),
                parse_scalar("0x58fc4353"),
                parse_scalar("0x1fa8695a"),
                parse_scalar("0x4a026a9d"),
                parse_scalar("0x3e0c59ec"),
                parse_scalar("0x3be00b5d"),
                parse_scalar("0x687cd26e"),
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
                parse_scalar("0x2cd28abd"),
                parse_scalar("0x4b7e0de6"),
                parse_scalar("0x1c28010b"),
                parse_scalar("0x341a4215"),
                parse_scalar("0x2730f966"),
                parse_scalar("0x6d5f6492"),
                parse_scalar("0x2da41e53"),
                parse_scalar("0x5b9553f8"),
                parse_scalar("0x40f3bd78"),
                parse_scalar("0x1847d418"),
                parse_scalar("0x3b40f194"),
                parse_scalar("0x61deec2d"),
                parse_scalar("0x268de447"),
                parse_scalar("0x4ff70519"),
                parse_scalar("0x0276d981"),
                parse_scalar("0x0ecd4498"),
                parse_scalar("0x5852c99b"),
                parse_scalar("0x0c0c5656"),
                parse_scalar("0x1a5d72b5"),
                parse_scalar("0x1556aa75"),
                parse_scalar("0x05df8f05"),
                parse_scalar("0x4e9c5eb7"),
                parse_scalar("0x669108c9"),
                parse_scalar("0x051bc5f5"),
                parse_scalar("0x5d193fdf"),
                parse_scalar("0x1fc5ddac"),
                parse_scalar("0x478e26aa"),
                parse_scalar("0x78fe0af3"),
                parse_scalar("0x40b6dd16"),
                parse_scalar("0x6cc4f791"),
                parse_scalar("0x77e872fe"),
                parse_scalar("0x6112c017"),
            ]
        );
    }

    // TODO
}
