use crate::params::decode_constants;
use crate::poseidon::{self, sbox5};
use starkom_ff::bls12_381::Scalar;
use std::sync::LazyLock;

/// Poseidon configuration for the BLS12-381 scalar field.
pub struct BlsConfig<const T: usize, const R: usize, const C: usize> {}

impl poseidon::Config<Scalar, 3, 2, 1> for BlsConfig<3, 2, 1> {
    fn num_full_rounds() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        56
    }

    fn num_total_rounds() -> usize {
        64
    }

    fn sbox(x: Scalar) -> Scalar {
        sbox5(x)
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 192]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bls12_381/arc_t3.bin");
            decode_constants::<Scalar, 192>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 9]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bls12_381/mds_t3.bin");
            decode_constants::<Scalar, 9>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 4, 3, 1> for BlsConfig<4, 3, 1> {
    fn num_full_rounds() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        56
    }

    fn num_total_rounds() -> usize {
        64
    }

    fn sbox(x: Scalar) -> Scalar {
        sbox5(x)
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 256]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bls12_381/arc_t4.bin");
            decode_constants::<Scalar, 256>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 16]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bls12_381/mds_t4.bin");
            decode_constants::<Scalar, 16>(bytes)
        });
        &*MATRIX
    }
}

/// Poseidon configuration for BLS12-381 with T=3.
pub type BlsConfig3 = BlsConfig<3, 2, 1>;

/// Poseidon configuration for BLS12-381 with T=4.
pub type BlsConfig4 = BlsConfig<4, 3, 1>;

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_scalar(s: &'static str) -> Scalar {
        s.parse().unwrap()
    }

    fn hash_t3(inputs: &[Scalar]) -> [Scalar; 3] {
        poseidon::hash::<BlsConfig3, Scalar, 3, 2, 1>(inputs)
    }

    fn hash_t3_0(inputs: &[Scalar]) -> Scalar {
        poseidon::hash0::<BlsConfig3, Scalar, 3, 2, 1>(inputs)
    }

    fn hash_t4(inputs: &[Scalar]) -> [Scalar; 4] {
        poseidon::hash::<BlsConfig4, Scalar, 4, 3, 1>(inputs)
    }

    fn hash_t4_0(inputs: &[Scalar]) -> Scalar {
        poseidon::hash0::<BlsConfig4, Scalar, 4, 3, 1>(inputs)
    }

    #[test]
    fn test_permutation_t3() {
        assert_eq!(
            poseidon::permutation::<BlsConfig3, Scalar, 3, 2, 1>([
                Scalar::from_const(0),
                Scalar::from_const(1),
                Scalar::from_const(2),
            ]),
            [
                parse_scalar("0x3fb8310b0e962b75bffec5f9cfcbf3f965a7b1d2dcac8d95ccb13d434e08e5fa"),
                parse_scalar("0x43fe5dfa886bfae59d015ed8b2a8c9328230f299203c89b9c78d8b40ccdc7dda"),
                parse_scalar("0x05153d5d7d0f9122550ecc902c0f5248d8ddcacfa1b911699c982099efc48aa7"),
            ]
        );
    }

    #[test]
    fn test_permutation_t4() {
        assert_eq!(
            poseidon::permutation::<BlsConfig4, Scalar, 4, 3, 1>([
                Scalar::from_const(0),
                Scalar::from_const(1),
                Scalar::from_const(2),
                Scalar::from_const(3),
            ]),
            [
                parse_scalar("0x5ad8bcfa9754b5bc043cc74dea65ae15e3fdb0c2295970aaacfc116c802d9895"),
                parse_scalar("0x03ed9e6e45c050ecfa18b36cb8fa3ad18247f12897a2cbdc4afd565d2f5d04d0"),
                parse_scalar("0x3feefc27c9dac582d1ef7a70d4fdc89ca20fddbebc1bf92781d142b71be23c10"),
                parse_scalar("0x15b696e71b1ae2d964b6cb41b41cec75f7fb9587571945300e9631e139fb0775"),
            ]
        );
    }

    #[test]
    fn test_hash_t3_1() {
        assert_eq!(
            hash_t3(&[Scalar::from_const(42)]),
            [
                parse_scalar("0x23cb77dbdf16c9f51569c9fe0aa06fbf21c54ac8f606896a14fb74e6c48af04c"),
                parse_scalar("0x5e3fabe504ab4da42267751948d02d5a0b79f23a86816b81ab481ab270ce6ccd"),
                parse_scalar("0x43da39505ac8f81c045d938f3ee5e8ffe6761e1960ee5bb9fc0fbc15a442319f"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[Scalar::from_const(42)]),
            parse_scalar("0x23cb77dbdf16c9f51569c9fe0aa06fbf21c54ac8f606896a14fb74e6c48af04c")
        );
    }

    #[test]
    fn test_hash_t3_2() {
        assert_eq!(
            hash_t3(&[Scalar::from_const(1), Scalar::from_const(2)]),
            [
                parse_scalar("0x7384612d0bb2ae1a7567ccde6ea63a0249dc451c7317a49c48b8a091e71ca335"),
                parse_scalar("0x52f2dd26c3a79aed9f7469200728bd01eb05f4eebd109031ffad98d4cdab813c"),
                parse_scalar("0x3181fd0d783b9ee1a31d3cac664b8e5de9a3b42e504b9422da131344c7aa2460"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[Scalar::from_const(1), Scalar::from_const(2)]),
            parse_scalar("0x7384612d0bb2ae1a7567ccde6ea63a0249dc451c7317a49c48b8a091e71ca335")
        );
    }

    #[test]
    fn test_hash_t3_3() {
        assert_eq!(
            hash_t3(&[
                Scalar::from_const(3),
                Scalar::from_const(4),
                Scalar::from_const(5),
            ]),
            [
                parse_scalar("0x239c914bda953deb1525c84c03bf73fd55c0b7c848664f44e13f241f1eb23919"),
                parse_scalar("0x134b65dd83bd4f049ae77dedf7ec6ca6fe1a782f3112af7b446641cd7dc500a3"),
                parse_scalar("0x4c38e10957db93e7d83a8173e7d652406dd3c207b113a2c69c030c879bfea1ba"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[
                Scalar::from_const(3),
                Scalar::from_const(4),
                Scalar::from_const(5),
            ]),
            parse_scalar("0x239c914bda953deb1525c84c03bf73fd55c0b7c848664f44e13f241f1eb23919")
        );
    }

    #[test]
    fn test_hash_t3_4() {
        assert_eq!(
            hash_t3(&[
                Scalar::from_const(6),
                Scalar::from_const(7),
                Scalar::from_const(8),
                Scalar::from_const(9),
            ]),
            [
                parse_scalar("0x35c6bb2d0425a7f5199bd6a8cba05ac197e542c0e31706679f046830fcd3db8a"),
                parse_scalar("0x6e4c00d7fac94cc392125d8f17e4113d08f8456ac1ab0080968f1c271b474bdb"),
                parse_scalar("0x05e4fb0586ae7bce7e3e3319281591ddc7b891fa13f5fdc48575ea1558fd383c"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[
                Scalar::from_const(6),
                Scalar::from_const(7),
                Scalar::from_const(8),
                Scalar::from_const(9),
            ]),
            parse_scalar("0x35c6bb2d0425a7f5199bd6a8cba05ac197e542c0e31706679f046830fcd3db8a")
        );
    }

    #[test]
    fn test_hash_t3_5() {
        assert_eq!(
            hash_t3(&[
                Scalar::from_const(10),
                Scalar::from_const(11),
                Scalar::from_const(12),
                Scalar::from_const(13),
                Scalar::from_const(14),
            ]),
            [
                parse_scalar("0x5c2b2ed1892886ef098b56ca88988c297bf3954aaffce9f8b8dafceb26aba841"),
                parse_scalar("0x49f51dd06dc94b4dc6d42ddefad87e4864fb7b905f9e9a4c7cf5724ab7b8c9bd"),
                parse_scalar("0x1d974406d04372dbf4ee69aac67c9ffdeeaf18712d9f0858063c25dc452f3590"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[
                Scalar::from_const(10),
                Scalar::from_const(11),
                Scalar::from_const(12),
                Scalar::from_const(13),
                Scalar::from_const(14),
            ]),
            parse_scalar("0x5c2b2ed1892886ef098b56ca88988c297bf3954aaffce9f8b8dafceb26aba841")
        );
    }

    #[test]
    fn test_hash_t4_1() {
        assert_eq!(
            hash_t4(&[Scalar::from_const(42)]),
            [
                parse_scalar("0x0531b2fa3c2aa794859d54c409ac6bf33a19981275bff625c5eeb8d1cc8d123c"),
                parse_scalar("0x4de146bfef1a920bfda9018b1a05cdc7f38d49c51c66dca6d7ac0eadf450d7a1"),
                parse_scalar("0x3baa19798505d449d802fa837cd2151dd59b82520867db72ff811f4cd7e01c81"),
                parse_scalar("0x6b19becb4030b0aa0844a458d668b204b0fde20ed0c39e15a73d4185a3f8c98c"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[Scalar::from_const(42)]),
            parse_scalar("0x0531b2fa3c2aa794859d54c409ac6bf33a19981275bff625c5eeb8d1cc8d123c")
        );
    }

    #[test]
    fn test_hash_t4_2() {
        assert_eq!(
            hash_t4(&[Scalar::from_const(1), Scalar::from_const(2)]),
            [
                parse_scalar("0x520651bc5804254d3306d30c7e3242e00f527bb7f39aedb7f828e346299bd91c"),
                parse_scalar("0x66978e6d726f9a4d5a9645e57906b4393f17297840b93ccba7547e4f46664cdb"),
                parse_scalar("0x47669414f2967f8bf1068766a7702049e6f19a0bb2ce99e8dd1b3004c1f3769a"),
                parse_scalar("0x4883668674972fc54b38313aa1be285e5d87597d0d27d66adf9f183136f79b4e"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[Scalar::from_const(1), Scalar::from_const(2)]),
            parse_scalar("0x520651bc5804254d3306d30c7e3242e00f527bb7f39aedb7f828e346299bd91c")
        );
    }

    #[test]
    fn test_hash_t4_3() {
        assert_eq!(
            hash_t4(&[
                Scalar::from_const(3),
                Scalar::from_const(4),
                Scalar::from_const(5),
            ]),
            [
                parse_scalar("0x1a9f84b2d90c7ec4efb7e8c38efddad5983245c1132434bb94c74d19eb04cb3a"),
                parse_scalar("0x27d3440c24462b00339149798201fe261c12d7574ab232af78f7c915cf5ca364"),
                parse_scalar("0x31a0a4db15faff2776bf01128e9dfa3dc6bd1ecdcac56313e645bab204dd6bda"),
                parse_scalar("0x1b1fbb0e8d53446fe1f93c795716c9c54f987da2f15e3c1999362560ab25144b"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[
                Scalar::from_const(3),
                Scalar::from_const(4),
                Scalar::from_const(5),
            ]),
            parse_scalar("0x1a9f84b2d90c7ec4efb7e8c38efddad5983245c1132434bb94c74d19eb04cb3a")
        );
    }

    #[test]
    fn test_hash_t4_4() {
        assert_eq!(
            hash_t4(&[
                Scalar::from_const(6),
                Scalar::from_const(7),
                Scalar::from_const(8),
                Scalar::from_const(9),
            ]),
            [
                parse_scalar("0x5497afdc8bc505782b08a63601eec9fa0e4037e61d06f453edff9a8ca1991b76"),
                parse_scalar("0x71b7d3f0b69622c9259681b41bacd80b9641a556d4c4e3511b39edc0020463e0"),
                parse_scalar("0x568cb5ff1d5edccfa2af660a0057c3c4b99f6eb9caad00c4955357401d822ea2"),
                parse_scalar("0x6100f02970f3d9af68a70f8bc3ba23acea0837fbf6515be0855e5c7de99a515e"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[
                Scalar::from_const(6),
                Scalar::from_const(7),
                Scalar::from_const(8),
                Scalar::from_const(9),
            ]),
            parse_scalar("0x5497afdc8bc505782b08a63601eec9fa0e4037e61d06f453edff9a8ca1991b76")
        );
    }

    #[test]
    fn test_hash_t4_5() {
        assert_eq!(
            hash_t4(&[
                Scalar::from_const(10),
                Scalar::from_const(11),
                Scalar::from_const(12),
                Scalar::from_const(13),
                Scalar::from_const(14),
            ]),
            [
                parse_scalar("0x0c8f1b5e59a0120bda56f3e28b2558f3541f2fc0a421418081b071dd30e89a3f"),
                parse_scalar("0x580ae06016546d5151fe07525174e59b512d2a8c028a4adc46a785a43f181755"),
                parse_scalar("0x3ef10618b58d716c5289a9f9c31f68c5f8d9da3d5cbd36307fc65b783975ec12"),
                parse_scalar("0x510f1f5fe9ca98d82d252785b761a794b916a63865fe486819de37383a68f78a"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[
                Scalar::from_const(10),
                Scalar::from_const(11),
                Scalar::from_const(12),
                Scalar::from_const(13),
                Scalar::from_const(14),
            ]),
            parse_scalar("0x0c8f1b5e59a0120bda56f3e28b2558f3541f2fc0a421418081b071dd30e89a3f")
        );
    }
}
