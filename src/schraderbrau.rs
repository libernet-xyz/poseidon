use crate::params::decode_constants;
use crate::poseidon;
use starkom_schraderbrau::Scalar;
use std::sync::LazyLock;

/// Poseidon configuration for the Schraderbrau scalar field.
pub struct SchraderbrauConfig<const T: usize> {}

impl poseidon::Config<Scalar, 3> for SchraderbrauConfig<3> {
    fn num_full_rounds() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        83
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 273]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/schraderbrau/arc_t3.bin");
            decode_constants::<Scalar, 273>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 9]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/schraderbrau/mds_t3.bin");
            decode_constants::<Scalar, 9>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 4> for SchraderbrauConfig<4> {
    fn num_full_rounds() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        84
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 368]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/schraderbrau/arc_t4.bin");
            decode_constants::<Scalar, 368>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 16]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/schraderbrau/mds_t4.bin");
            decode_constants::<Scalar, 16>(bytes)
        });
        &*MATRIX
    }
}

/// Poseidon configuration for Schraderbrau with T=3.
pub type SchraderbrauConfig3 = SchraderbrauConfig<3>;

/// Poseidon configuration for Schraderbrau with T=4.
pub type SchraderbrauConfig4 = SchraderbrauConfig<4>;

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_scalar(s: &'static str) -> Scalar {
        s.parse().unwrap()
    }

    fn hash_t3(inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 2] {
        poseidon::hash::<SchraderbrauConfig3, Scalar, 3, 2, 1>(inputs)
    }

    fn hash_t3_0(inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<SchraderbrauConfig3, Scalar, 3, 2, 1>(inputs)
    }

    fn hash_t4(inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 3] {
        poseidon::hash::<SchraderbrauConfig4, Scalar, 4, 3, 1>(inputs)
    }

    fn hash_t4_0(inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<SchraderbrauConfig4, Scalar, 4, 3, 1>(inputs)
    }

    #[test]
    fn test_permutation_t3() {
        assert_eq!(
            poseidon::permutation::<SchraderbrauConfig3, Scalar, 3>([
                Scalar::from_const(0),
                Scalar::from_const(1),
                Scalar::from_const(2),
            ]),
            [
                parse_scalar("0x1531e124d8f663b8d26c56b9bf0b1b09ba15d4e20ea10a3d898cdcf1d2c41dba"),
                parse_scalar("0x1f5714cc13f8a33f4b32a07f1a85409de2d1b8d353aa269ca8f2bfa91071bd62"),
                parse_scalar("0x588c20c682f8b66c52049d910a4f0e7195dfd300a0d0cf3198b1383be1a4134a"),
            ]
        );
    }

    #[test]
    fn test_permutation_t4() {
        assert_eq!(
            poseidon::permutation::<SchraderbrauConfig4, Scalar, 4>([
                Scalar::from_const(0),
                Scalar::from_const(1),
                Scalar::from_const(2),
                Scalar::from_const(3),
            ]),
            [
                parse_scalar("0x11de0b3702563747b0729abd2b93e720ec947ce067ca3f8c088b9829df1169d7"),
                parse_scalar("0x487ca87fa054ad960f6f571cf7aa6f5e075fd5ac3386e2e9fb22aeeb036fb1e1"),
                parse_scalar("0x28c775ddba9039531ee5deb0d4e2e6b1c9bb7d8be8da260a20d331bb041d0d38"),
                parse_scalar("0x10c93f76dca6f63507bc8ed1d2ea40647bb480ad43ac9922a3f10597b802f949"),
            ]
        );
    }

    #[test]
    fn test_hash_t3_1() {
        assert_eq!(
            hash_t3([Scalar::from_const(42)]),
            [
                parse_scalar("0x39ae885d9e63b2ba90b6dc9a46feac9f63987fc5e0ec54c270315176cd710378"),
                parse_scalar("0x6f1a3ef89afe9b23e7b38d4dd96d6f62f4c6025f842b6324921d0ce7452ab7ff"),
            ]
        );
        assert_eq!(
            hash_t3_0([Scalar::from_const(42)]),
            parse_scalar("0x39ae885d9e63b2ba90b6dc9a46feac9f63987fc5e0ec54c270315176cd710378")
        );
    }

    #[test]
    fn test_hash_t3_2() {
        assert_eq!(
            hash_t3([Scalar::from_const(1), Scalar::from_const(2)]),
            [
                parse_scalar("0x61515087e040a0adf1407e0f4b9b1ce7104b4cb615c526edeb9792c80738b043"),
                parse_scalar("0x146c0f3648d474f9ec85ed2dc1ffea1aec4d032b620980814fc9cdb0324bae24"),
            ]
        );
        assert_eq!(
            hash_t3_0([Scalar::from_const(1), Scalar::from_const(2)]),
            parse_scalar("0x61515087e040a0adf1407e0f4b9b1ce7104b4cb615c526edeb9792c80738b043")
        );
    }

    #[test]
    fn test_hash_t3_3() {
        assert_eq!(
            hash_t3([
                Scalar::from_const(3),
                Scalar::from_const(4),
                Scalar::from_const(5),
            ]),
            [
                parse_scalar("0x0ef6e70b7c69836d110a2fd848a253bf709be6d686946a36790dc267c44f378c"),
                parse_scalar("0x18e6a853d074b2a46d9bd2051ca51ed1cf1a513d6f5922a712c3c81115ee7b98"),
            ]
        );
        assert_eq!(
            hash_t3_0([
                Scalar::from_const(3),
                Scalar::from_const(4),
                Scalar::from_const(5),
            ]),
            parse_scalar("0x0ef6e70b7c69836d110a2fd848a253bf709be6d686946a36790dc267c44f378c")
        );
    }

    #[test]
    fn test_hash_t3_4() {
        assert_eq!(
            hash_t3([
                Scalar::from_const(6),
                Scalar::from_const(7),
                Scalar::from_const(8),
                Scalar::from_const(9),
            ]),
            [
                parse_scalar("0x6f0d33f7982cfb50c75397e2c7efe40d4bd2c79bc8d16e66df81dd8c1731996e"),
                parse_scalar("0x49e031c765b6b63ec61c0445957e724a02f3b6b956a4973d6d3f3fea8e4466e0"),
            ]
        );
        assert_eq!(
            hash_t3_0([
                Scalar::from_const(6),
                Scalar::from_const(7),
                Scalar::from_const(8),
                Scalar::from_const(9),
            ]),
            parse_scalar("0x6f0d33f7982cfb50c75397e2c7efe40d4bd2c79bc8d16e66df81dd8c1731996e")
        );
    }

    #[test]
    fn test_hash_t3_5() {
        assert_eq!(
            hash_t3([
                Scalar::from_const(10),
                Scalar::from_const(11),
                Scalar::from_const(12),
                Scalar::from_const(13),
                Scalar::from_const(14),
            ]),
            [
                parse_scalar("0x22fea6517989ccb8e243c7a44c35315b1eceb2bd5fdaedb174f09851ef920115"),
                parse_scalar("0x46c63429c51bd18252cdb7bd83eaffb137c9032a05c76fc0915c6be8c82138ab"),
            ]
        );
        assert_eq!(
            hash_t3_0([
                Scalar::from_const(10),
                Scalar::from_const(11),
                Scalar::from_const(12),
                Scalar::from_const(13),
                Scalar::from_const(14),
            ]),
            parse_scalar("0x22fea6517989ccb8e243c7a44c35315b1eceb2bd5fdaedb174f09851ef920115")
        );
    }

    #[test]
    fn test_hash_t4_1() {
        assert_eq!(
            hash_t4([Scalar::from_const(42)]),
            [
                parse_scalar("0x574545df9bcb4a291d90a35b491d8f358387f7631a03d7abb5402d0dd741012f"),
                parse_scalar("0x7f25a9b1edffcc079df1165b54fcc27295a15f24449dd83d9686c88972852790"),
                parse_scalar("0x0912a27650b3385c42ad5825b81a885f5e80a8eb56b5d31c1f4f16a0dc0a1354"),
            ]
        );
        assert_eq!(
            hash_t4_0([Scalar::from_const(42)]),
            parse_scalar("0x574545df9bcb4a291d90a35b491d8f358387f7631a03d7abb5402d0dd741012f")
        );
    }

    #[test]
    fn test_hash_t4_2() {
        assert_eq!(
            hash_t4([Scalar::from_const(1), Scalar::from_const(2)]),
            [
                parse_scalar("0x3e17210c19dd4e618e0b6a7964569f712378b278611822e83a90de34f8bd7718"),
                parse_scalar("0x0d3b4ee0c17cdf7f251f65403bf663de414389777ffca648ff3d895fba943901"),
                parse_scalar("0x187e0d21e2e75b7f63c22444bceac7742c72ffb9523579104b6ed7bd0502a1af"),
            ]
        );
        assert_eq!(
            hash_t4_0([Scalar::from_const(1), Scalar::from_const(2)]),
            parse_scalar("0x3e17210c19dd4e618e0b6a7964569f712378b278611822e83a90de34f8bd7718")
        );
    }

    #[test]
    fn test_hash_t4_3() {
        assert_eq!(
            hash_t4([
                Scalar::from_const(3),
                Scalar::from_const(4),
                Scalar::from_const(5),
            ]),
            [
                parse_scalar("0x169474a88b5e70380cd3b5f35a48300d3a7c5aca7b58ca06922d21093b2fd2b4"),
                parse_scalar("0x01991694a2f344508f9b52185f45f256a63097d4c61187c33bdf87a4dd7bfcaa"),
                parse_scalar("0x20e757bf7868907226f471e22aaf931939a03e0cc5ae0a46918739fbbf83bebf"),
            ]
        );
        assert_eq!(
            hash_t4_0([
                Scalar::from_const(3),
                Scalar::from_const(4),
                Scalar::from_const(5),
            ]),
            parse_scalar("0x169474a88b5e70380cd3b5f35a48300d3a7c5aca7b58ca06922d21093b2fd2b4")
        );
    }

    #[test]
    fn test_hash_t4_4() {
        assert_eq!(
            hash_t4([
                Scalar::from_const(6),
                Scalar::from_const(7),
                Scalar::from_const(8),
                Scalar::from_const(9),
            ]),
            [
                parse_scalar("0x4f6de8af3f5427c906459a8081a755dc2b6817e7f0d4e498ebe45357bf1f0a55"),
                parse_scalar("0x34c508dffafebb5aec932ef28ca7d1060076aa85d8cb4dfb88330a996a2ef4a5"),
                parse_scalar("0x0455ca69680c67b3bbd4ee3c50f8f61862a5789967cdd2badf075fd846c7c934"),
            ]
        );
        assert_eq!(
            hash_t4_0([
                Scalar::from_const(6),
                Scalar::from_const(7),
                Scalar::from_const(8),
                Scalar::from_const(9),
            ]),
            parse_scalar("0x4f6de8af3f5427c906459a8081a755dc2b6817e7f0d4e498ebe45357bf1f0a55")
        );
    }

    #[test]
    fn test_hash_t4_5() {
        assert_eq!(
            hash_t4([
                Scalar::from_const(10),
                Scalar::from_const(11),
                Scalar::from_const(12),
                Scalar::from_const(13),
                Scalar::from_const(14),
            ]),
            [
                parse_scalar("0x63ab44ef4f66e3bc8401c5421bb3c187b1a25e26d5f6a07a3c56b2f03e6bd9e2"),
                parse_scalar("0x02999f80700822cdcda793d9bccb2beae348d5028d3ee13474341f066152a984"),
                parse_scalar("0x62dc53498ab59b7e293991880a5138cc06b59a5b0ad4b9d3345929ffc5402486"),
            ]
        );
        assert_eq!(
            hash_t4_0([
                Scalar::from_const(10),
                Scalar::from_const(11),
                Scalar::from_const(12),
                Scalar::from_const(13),
                Scalar::from_const(14),
            ]),
            parse_scalar("0x63ab44ef4f66e3bc8401c5421bb3c187b1a25e26d5f6a07a3c56b2f03e6bd9e2")
        );
    }
}
