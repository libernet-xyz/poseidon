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
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
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
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
                parse_scalar("0x00000000"),
            ]
        );
    }

    // TODO
}
