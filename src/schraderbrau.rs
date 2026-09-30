use crate::params::decode_constants;
use crate::poseidon;
use starkom_schraderbrau::Scalar;
use std::sync::LazyLock;

/// Poseidon configuration for the Schraderbrau scalar field.
pub struct SchraderbrauConfig<const T: usize> {}

impl poseidon::Config<Scalar, 3> for SchraderbrauConfig<3> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        83
    }

    fn alpha() -> usize {
        3
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
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        84
    }

    fn alpha() -> usize {
        3
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

impl poseidon::Config<Scalar, 5> for SchraderbrauConfig<5> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        84
    }

    fn alpha() -> usize {
        3
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 460]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/schraderbrau/arc_t5.bin");
            decode_constants::<Scalar, 460>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 25]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/schraderbrau/mds_t5.bin");
            decode_constants::<Scalar, 25>(bytes)
        });
        &*MATRIX
    }
}

/// Poseidon configuration for Schraderbrau with T=3.
pub type SchraderbrauConfig3 = SchraderbrauConfig<3>;

/// Poseidon configuration for Schraderbrau with T=4.
pub type SchraderbrauConfig4 = SchraderbrauConfig<4>;

/// Poseidon configuration for Schraderbrau with T=5.
pub type SchraderbrauConfig5 = SchraderbrauConfig<5>;

#[cfg(test)]
mod tests {
    use super::*;
    use starkom_ff::Field;
    use starkom_schraderbrau::{from_const, parse_scalar};

    const DST: Scalar = Scalar::ZERO;

    fn hash_t3(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 2] {
        poseidon::hash::<SchraderbrauConfig3, Scalar, 3, 2, 1>([dst], inputs)
    }

    fn hash_t3_0(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<SchraderbrauConfig3, Scalar, 3, 2, 1>([dst], inputs)
    }

    fn hash_t4(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 3] {
        poseidon::hash::<SchraderbrauConfig4, Scalar, 4, 3, 1>([dst], inputs)
    }

    fn hash_t4_0(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<SchraderbrauConfig4, Scalar, 4, 3, 1>([dst], inputs)
    }

    fn hash_t5(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 4] {
        poseidon::hash::<SchraderbrauConfig5, Scalar, 5, 4, 1>([dst], inputs)
    }

    fn hash_t5_0(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<SchraderbrauConfig5, Scalar, 5, 4, 1>([dst], inputs)
    }

    #[test]
    fn test_permutation_t3() {
        assert_eq!(
            poseidon::permutation::<SchraderbrauConfig3, Scalar, 3>([
                from_const(0),
                from_const(1),
                from_const(2),
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
                from_const(0),
                from_const(1),
                from_const(2),
                from_const(3),
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
    fn test_permutation_t5() {
        assert_eq!(
            poseidon::permutation::<SchraderbrauConfig5, Scalar, 5>([
                from_const(0),
                from_const(1),
                from_const(2),
                from_const(3),
                from_const(4),
            ]),
            [
                parse_scalar("0x670aeee576a52920508f11f7ac7cb1be06997a2652aa27dd3c80a067539fc675"),
                parse_scalar("0x6d958b549642764a34bdbe48c37c247d936bfb583f58f926aac64c2e190c96d5"),
                parse_scalar("0x24fdd5991cad37acde42623ffda322c6f7faad0efd5a8a3e629b44ce231a4f24"),
                parse_scalar("0x68b182017cd4884c69f2097988e85c0e6145edd018b20c3774254935372264c4"),
                parse_scalar("0x7bcb1dd03d6e1da1324bbe54212579056d859d5f8303b828375ed3527b5bb3c9"),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t3() {
        assert_eq!(
            hash_t3(DST, [from_const(0), from_const(1)]),
            [
                parse_scalar("0x0bdee2ba4babd015e992e98174d1f0ca2c59f3c14125fb126c753ae2d43ff0c6"),
                parse_scalar("0x78dcdf2961206c9583adee397c1095734740b445f81629f03c07cb99961c748c"),
            ]
        );
        assert_eq!(
            hash_t3(from_const(2), [from_const(0), from_const(1)]),
            [
                parse_scalar("0x1531e124d8f663b8d26c56b9bf0b1b09ba15d4e20ea10a3d898cdcf1d2c41dba"),
                parse_scalar("0x1f5714cc13f8a33f4b32a07f1a85409de2d1b8d353aa269ca8f2bfa91071bd62"),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t4() {
        assert_eq!(
            hash_t4(DST, [from_const(0), from_const(1), from_const(2)]),
            [
                parse_scalar("0x6673d1e717a02404d2b1dbe1b4048dea7ad101885b10d1c1cddbdb289d82f0aa"),
                parse_scalar("0x52d44e0b2320a1adeb7f2b70cf71accfafee26668e360af52bbe828bfc0ac737"),
                parse_scalar("0x3e3574a2da7f37a5e5856132c4c63786d378ab251cf1deabb2f6ad4041e791f1"),
            ]
        );
        assert_eq!(
            hash_t4(from_const(3), [from_const(0), from_const(1), from_const(2)]),
            [
                parse_scalar("0x11de0b3702563747b0729abd2b93e720ec947ce067ca3f8c088b9829df1169d7"),
                parse_scalar("0x487ca87fa054ad960f6f571cf7aa6f5e075fd5ac3386e2e9fb22aeeb036fb1e1"),
                parse_scalar("0x28c775ddba9039531ee5deb0d4e2e6b1c9bb7d8be8da260a20d331bb041d0d38"),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t5() {
        assert_eq!(
            hash_t5(
                DST,
                [from_const(0), from_const(1), from_const(2), from_const(3)]
            ),
            [
                parse_scalar("0x4865e43ce2400cc962bd47291650efdc07d8258e38656c232f46869711e81ba1"),
                parse_scalar("0x42fb7f1d4bae8f32970c0bf2e1a57ef0255c7e82d08bdc358a2aabf115526977"),
                parse_scalar("0x6c9ca5f2eed5604ac2683e0da5c6f59b8807ef6d03403d6bc4234ff74481012f"),
                parse_scalar("0x29e7c947af53d8c1ecede67e0f9fe9b4c2f832522b2678391b003c0d5545ef38"),
            ]
        );
        assert_eq!(
            hash_t5(
                from_const(4),
                [from_const(0), from_const(1), from_const(2), from_const(3)]
            ),
            [
                parse_scalar("0x670aeee576a52920508f11f7ac7cb1be06997a2652aa27dd3c80a067539fc675"),
                parse_scalar("0x6d958b549642764a34bdbe48c37c247d936bfb583f58f926aac64c2e190c96d5"),
                parse_scalar("0x24fdd5991cad37acde42623ffda322c6f7faad0efd5a8a3e629b44ce231a4f24"),
                parse_scalar("0x68b182017cd4884c69f2097988e85c0e6145edd018b20c3774254935372264c4"),
            ]
        );
    }

    #[test]
    fn test_hash_t3_1() {
        assert_eq!(
            hash_t3(DST, [from_const(42)]),
            [
                parse_scalar("0x39ae885d9e63b2ba90b6dc9a46feac9f63987fc5e0ec54c270315176cd710378"),
                parse_scalar("0x6f1a3ef89afe9b23e7b38d4dd96d6f62f4c6025f842b6324921d0ce7452ab7ff"),
            ]
        );
        assert_eq!(
            hash_t3_0(DST, [from_const(42)]),
            parse_scalar("0x39ae885d9e63b2ba90b6dc9a46feac9f63987fc5e0ec54c270315176cd710378")
        );
    }

    #[test]
    fn test_hash_t3_2() {
        assert_eq!(
            hash_t3(DST, [from_const(1), from_const(2)]),
            [
                parse_scalar("0x61515087e040a0adf1407e0f4b9b1ce7104b4cb615c526edeb9792c80738b043"),
                parse_scalar("0x146c0f3648d474f9ec85ed2dc1ffea1aec4d032b620980814fc9cdb0324bae24"),
            ]
        );
        assert_eq!(
            hash_t3_0(DST, [from_const(1), from_const(2)]),
            parse_scalar("0x61515087e040a0adf1407e0f4b9b1ce7104b4cb615c526edeb9792c80738b043")
        );
    }

    #[test]
    fn test_hash_t3_3() {
        assert_eq!(
            hash_t3(DST, [from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x0ef6e70b7c69836d110a2fd848a253bf709be6d686946a36790dc267c44f378c"),
                parse_scalar("0x18e6a853d074b2a46d9bd2051ca51ed1cf1a513d6f5922a712c3c81115ee7b98"),
            ]
        );
        assert_eq!(
            hash_t3_0(DST, [from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x0ef6e70b7c69836d110a2fd848a253bf709be6d686946a36790dc267c44f378c")
        );
    }

    #[test]
    fn test_hash_t3_4() {
        assert_eq!(
            hash_t3(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            [
                parse_scalar("0x6f0d33f7982cfb50c75397e2c7efe40d4bd2c79bc8d16e66df81dd8c1731996e"),
                parse_scalar("0x49e031c765b6b63ec61c0445957e724a02f3b6b956a4973d6d3f3fea8e4466e0"),
            ]
        );
        assert_eq!(
            hash_t3_0(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            parse_scalar("0x6f0d33f7982cfb50c75397e2c7efe40d4bd2c79bc8d16e66df81dd8c1731996e")
        );
    }

    #[test]
    fn test_hash_t3_5() {
        assert_eq!(
            hash_t3(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14),
                ]
            ),
            [
                parse_scalar("0x22fea6517989ccb8e243c7a44c35315b1eceb2bd5fdaedb174f09851ef920115"),
                parse_scalar("0x46c63429c51bd18252cdb7bd83eaffb137c9032a05c76fc0915c6be8c82138ab"),
            ]
        );
        assert_eq!(
            hash_t3_0(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14),
                ]
            ),
            parse_scalar("0x22fea6517989ccb8e243c7a44c35315b1eceb2bd5fdaedb174f09851ef920115")
        );
    }

    #[test]
    fn test_hash_t4_1() {
        assert_eq!(
            hash_t4(DST, [from_const(42)]),
            [
                parse_scalar("0x574545df9bcb4a291d90a35b491d8f358387f7631a03d7abb5402d0dd741012f"),
                parse_scalar("0x7f25a9b1edffcc079df1165b54fcc27295a15f24449dd83d9686c88972852790"),
                parse_scalar("0x0912a27650b3385c42ad5825b81a885f5e80a8eb56b5d31c1f4f16a0dc0a1354"),
            ]
        );
        assert_eq!(
            hash_t4_0(DST, [from_const(42)]),
            parse_scalar("0x574545df9bcb4a291d90a35b491d8f358387f7631a03d7abb5402d0dd741012f")
        );
    }

    #[test]
    fn test_hash_t4_2() {
        assert_eq!(
            hash_t4(DST, [from_const(1), from_const(2)]),
            [
                parse_scalar("0x3e17210c19dd4e618e0b6a7964569f712378b278611822e83a90de34f8bd7718"),
                parse_scalar("0x0d3b4ee0c17cdf7f251f65403bf663de414389777ffca648ff3d895fba943901"),
                parse_scalar("0x187e0d21e2e75b7f63c22444bceac7742c72ffb9523579104b6ed7bd0502a1af"),
            ]
        );
        assert_eq!(
            hash_t4_0(DST, [from_const(1), from_const(2)]),
            parse_scalar("0x3e17210c19dd4e618e0b6a7964569f712378b278611822e83a90de34f8bd7718")
        );
    }

    #[test]
    fn test_hash_t4_3() {
        assert_eq!(
            hash_t4(DST, [from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x169474a88b5e70380cd3b5f35a48300d3a7c5aca7b58ca06922d21093b2fd2b4"),
                parse_scalar("0x01991694a2f344508f9b52185f45f256a63097d4c61187c33bdf87a4dd7bfcaa"),
                parse_scalar("0x20e757bf7868907226f471e22aaf931939a03e0cc5ae0a46918739fbbf83bebf"),
            ]
        );
        assert_eq!(
            hash_t4_0(DST, [from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x169474a88b5e70380cd3b5f35a48300d3a7c5aca7b58ca06922d21093b2fd2b4")
        );
    }

    #[test]
    fn test_hash_t4_4() {
        assert_eq!(
            hash_t4(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            [
                parse_scalar("0x4f6de8af3f5427c906459a8081a755dc2b6817e7f0d4e498ebe45357bf1f0a55"),
                parse_scalar("0x34c508dffafebb5aec932ef28ca7d1060076aa85d8cb4dfb88330a996a2ef4a5"),
                parse_scalar("0x0455ca69680c67b3bbd4ee3c50f8f61862a5789967cdd2badf075fd846c7c934"),
            ]
        );
        assert_eq!(
            hash_t4_0(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            parse_scalar("0x4f6de8af3f5427c906459a8081a755dc2b6817e7f0d4e498ebe45357bf1f0a55")
        );
    }

    #[test]
    fn test_hash_t4_5() {
        assert_eq!(
            hash_t4(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14),
                ]
            ),
            [
                parse_scalar("0x63ab44ef4f66e3bc8401c5421bb3c187b1a25e26d5f6a07a3c56b2f03e6bd9e2"),
                parse_scalar("0x02999f80700822cdcda793d9bccb2beae348d5028d3ee13474341f066152a984"),
                parse_scalar("0x62dc53498ab59b7e293991880a5138cc06b59a5b0ad4b9d3345929ffc5402486"),
            ]
        );
        assert_eq!(
            hash_t4_0(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14),
                ]
            ),
            parse_scalar("0x63ab44ef4f66e3bc8401c5421bb3c187b1a25e26d5f6a07a3c56b2f03e6bd9e2")
        );
    }

    #[test]
    fn test_hash_t5_1() {
        assert_eq!(
            hash_t5(DST, [from_const(42)]),
            [
                parse_scalar("0x301559076578a8d9b8f3ff28c387cf8e1c44a6cb90f8006882de94fc499bd78d"),
                parse_scalar("0x4983abb39db772203d1188cda01720e9dc5da1716fbad087157e1bd4e0de7855"),
                parse_scalar("0x0e99c762ff39c482c7a2a0dda24aa2886519534c092dd9b5fafd46027dfe506e"),
                parse_scalar("0x4c28e7e4bce050e540aa9ef8b1f0dad4610680da85516e7ca8868e0e2a657ad0"),
            ]
        );
        assert_eq!(
            hash_t5_0(DST, [from_const(42)]),
            parse_scalar("0x301559076578a8d9b8f3ff28c387cf8e1c44a6cb90f8006882de94fc499bd78d")
        );
        assert_eq!(
            hash_t5(from_const(42), [from_const(42)]),
            [
                parse_scalar("0x4c0ef9f661d6d633694607b4d6a54d2d417bf8a8419d5bc9592b10c9a297da10"),
                parse_scalar("0x7793b1eecec7e9025fb13df55159f8811792e211e7f30db847c886c05afa6c0c"),
                parse_scalar("0x7fa415a33e8899cecaac4e5784f6da2eca92abfd19e495290afeb71bb766b109"),
                parse_scalar("0x2596d19198126a74500bbac8ad4a8d0dbcc5cd485ed07795d369817927d9aae1"),
            ]
        );
        assert_eq!(
            hash_t5_0(from_const(42), [from_const(42)]),
            parse_scalar("0x4c0ef9f661d6d633694607b4d6a54d2d417bf8a8419d5bc9592b10c9a297da10")
        );
    }

    #[test]
    fn test_hash_t5_2() {
        assert_eq!(
            hash_t5(DST, [from_const(1), from_const(2)]),
            [
                parse_scalar("0x29b5b4122420c40e6994e95f64c9e6b3f072029e5a0db7bdb91a8f917393deb6"),
                parse_scalar("0x306a582182be540ba6728364168b3f4af28fa214ee6318f950d84bd81bf2de20"),
                parse_scalar("0x547a3533b2f371032fab8690e99fd98d2cd2314691971cfe1608c5e1fdb8216a"),
                parse_scalar("0x034b886fd3866c2e2be8ef223157519e016680bc98ab4992fe2f129f43107fa2"),
            ]
        );
        assert_eq!(
            hash_t5_0(DST, [from_const(1), from_const(2)]),
            parse_scalar("0x29b5b4122420c40e6994e95f64c9e6b3f072029e5a0db7bdb91a8f917393deb6")
        );
    }

    #[test]
    fn test_hash_t5_3() {
        assert_eq!(
            hash_t5(DST, [from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x0c9ad349fc074bdea3bf200c31055b1b43a77e86fbdc27e8c370607a27d5d2aa"),
                parse_scalar("0x43550f8a02c2d474faa98a4b5fd5b6444dfa3170255c6a1e30b1913346a6ed9c"),
                parse_scalar("0x467755433e62876d3aa07f3e4fc38a9fcb69e33000099be6e1befea673aa96ce"),
                parse_scalar("0x079d7f07539a41c7d7de76279e0f7da22b9786b6a4b63e699a809f55a7e97cc3"),
            ]
        );
        assert_eq!(
            hash_t5_0(DST, [from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x0c9ad349fc074bdea3bf200c31055b1b43a77e86fbdc27e8c370607a27d5d2aa")
        );
    }

    #[test]
    fn test_hash_t5_4() {
        assert_eq!(
            hash_t5(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            [
                parse_scalar("0x0e73dce0ba1d3c4b8c080ae899aabdc15573ab0a3af7ec61a28cb09238f6e7af"),
                parse_scalar("0x6da164224101adc754c9f9ba1e47a9cb9d77014aa885a45d0c2c1559c3bf039b"),
                parse_scalar("0x69ef759c7e7b8632a8513817791640fa7eb36ae41ec209acf104fa61614cf6ba"),
                parse_scalar("0x23a1dc691dc4f0b55506bb475481c510aeae4c074c0bc890838a7891e5cd5164"),
            ]
        );
        assert_eq!(
            hash_t5_0(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            parse_scalar("0x0e73dce0ba1d3c4b8c080ae899aabdc15573ab0a3af7ec61a28cb09238f6e7af")
        );
    }

    #[test]
    fn test_hash_t5_5() {
        assert_eq!(
            hash_t5(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14)
                ]
            ),
            [
                parse_scalar("0x23b9d04cb56fb7d2a47e14844c812b146917e68a8426140639d439ee5cff4f3c"),
                parse_scalar("0x4b346167b2a172e29919b83c44d782b637020e4238dbef5f7c7552bd7a9231fe"),
                parse_scalar("0x236c530c07fb1afe8e7c7b9fae6f90ae22144e759d5bea3e42b6d9991225c44b"),
                parse_scalar("0x450e96237aa256807c0caf11302348fb17562e027c4130d8bc170b9ab28a2f65"),
            ]
        );
        assert_eq!(
            hash_t5_0(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14)
                ]
            ),
            parse_scalar("0x23b9d04cb56fb7d2a47e14844c812b146917e68a8426140639d439ee5cff4f3c")
        );
    }
}
