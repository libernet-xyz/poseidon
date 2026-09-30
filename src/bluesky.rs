use crate::params::decode_constants;
use crate::poseidon;
use starkom_bluesky::Scalar;
use std::sync::LazyLock;

/// Poseidon configuration for the BlueSky field using the x^5 S-box.
pub struct BlueSkyConfigX5<const T: usize> {}

impl poseidon::Config<Scalar, 3> for BlueSkyConfigX5<3> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        56
    }

    fn alpha() -> usize {
        5
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 192]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/arc_t3_x5.bin");
            decode_constants::<Scalar, 192>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 9]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/mds_t3_x5.bin");
            decode_constants::<Scalar, 9>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 4> for BlueSkyConfigX5<4> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        56
    }

    fn alpha() -> usize {
        5
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 256]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/arc_t4_x5.bin");
            decode_constants::<Scalar, 256>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 16]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/mds_t4_x5.bin");
            decode_constants::<Scalar, 16>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 5> for BlueSkyConfigX5<5> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        56
    }

    fn alpha() -> usize {
        5
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 320]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/arc_t5_x5.bin");
            decode_constants::<Scalar, 320>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 25]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/mds_t5_x5.bin");
            decode_constants::<Scalar, 25>(bytes)
        });
        &*MATRIX
    }
}

/// Poseidon configuration for BlueSky with T=3 and x^5 S-box.
pub type BlueSkyConfigT3X5 = BlueSkyConfigX5<3>;

/// Poseidon configuration for BlueSky with T=4 and x^5 S-box.
pub type BlueSkyConfigT4X5 = BlueSkyConfigX5<4>;

/// Poseidon configuration for BlueSky with T=5 and x^5 S-box.
pub type BlueSkyConfigT5X5 = BlueSkyConfigX5<5>;

/// Poseidon configuration for the BlueSky field using the x^7 S-box.
pub struct BlueSkyConfigX7<const T: usize> {}

impl poseidon::Config<Scalar, 3> for BlueSkyConfigX7<3> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        46
    }

    fn alpha() -> usize {
        7
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 162]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/arc_t3_x7.bin");
            decode_constants::<Scalar, 162>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 9]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/mds_t3_x7.bin");
            decode_constants::<Scalar, 9>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 4> for BlueSkyConfigX7<4> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        46
    }

    fn alpha() -> usize {
        7
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 216]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/arc_t4_x7.bin");
            decode_constants::<Scalar, 216>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 16]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/mds_t4_x7.bin");
            decode_constants::<Scalar, 16>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 5> for BlueSkyConfigX7<5> {
    fn num_full_rounds_per_side() -> usize {
        4
    }

    fn num_partial_rounds() -> usize {
        46
    }

    fn alpha() -> usize {
        7
    }

    fn get_round_constants() -> &'static [Scalar] {
        static ROUND_CONSTANTS: LazyLock<[Scalar; 270]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/arc_t5_x7.bin");
            decode_constants::<Scalar, 270>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 25]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/mds_t5_x7.bin");
            decode_constants::<Scalar, 25>(bytes)
        });
        &*MATRIX
    }
}

/// Poseidon configuration for BlueSky with T=3 and x^7 S-box.
pub type BlueSkyConfigT3X7 = BlueSkyConfigX7<3>;

/// Poseidon configuration for BlueSky with T=4 and x^7 S-box.
pub type BlueSkyConfigT4X7 = BlueSkyConfigX7<4>;

/// Poseidon configuration for BlueSky with T=5 and x^7 S-box.
pub type BlueSkyConfigT5X7 = BlueSkyConfigX7<5>;

#[cfg(test)]
mod tests {
    use super::*;
    use starkom_bluesky::{from_const, parse_scalar};
    use starkom_ff::Field;

    const DST: Scalar = Scalar::ZERO;

    fn hash_t3_x5(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 2] {
        poseidon::hash::<BlueSkyConfigT3X5, Scalar, 3, 2, 1>([dst], inputs)
    }

    fn hash_t3_x5_0(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<BlueSkyConfigT3X5, Scalar, 3, 2, 1>([dst], inputs)
    }

    fn hash_t4_x5(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 3] {
        poseidon::hash::<BlueSkyConfigT4X5, Scalar, 4, 3, 1>([dst], inputs)
    }

    fn hash_t4_x5_0(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<BlueSkyConfigT4X5, Scalar, 4, 3, 1>([dst], inputs)
    }

    fn hash_t5_x5(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 4] {
        poseidon::hash::<BlueSkyConfigT5X5, Scalar, 5, 4, 1>([dst], inputs)
    }

    fn hash_t5_x5_0(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<BlueSkyConfigT5X5, Scalar, 5, 4, 1>([dst], inputs)
    }

    fn hash_t3_x7(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 2] {
        poseidon::hash::<BlueSkyConfigT3X7, Scalar, 3, 2, 1>([dst], inputs)
    }

    fn hash_t3_x7_0(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<BlueSkyConfigT3X7, Scalar, 3, 2, 1>([dst], inputs)
    }

    fn hash_t4_x7(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 3] {
        poseidon::hash::<BlueSkyConfigT4X7, Scalar, 4, 3, 1>([dst], inputs)
    }

    fn hash_t4_x7_0(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<BlueSkyConfigT4X7, Scalar, 4, 3, 1>([dst], inputs)
    }

    fn hash_t5_x7(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> [Scalar; 4] {
        poseidon::hash::<BlueSkyConfigT5X7, Scalar, 5, 4, 1>([dst], inputs)
    }

    fn hash_t5_x7_0(dst: Scalar, inputs: impl IntoIterator<Item = Scalar>) -> Scalar {
        poseidon::hash0::<BlueSkyConfigT5X7, Scalar, 5, 4, 1>([dst], inputs)
    }

    #[test]
    fn test_permutation_t3_x5() {
        assert_eq!(
            poseidon::permutation::<BlueSkyConfigT3X5, Scalar, 3>([
                from_const(0),
                from_const(1),
                from_const(2),
            ]),
            [
                parse_scalar("0x7b68dcd80fa751ee8f2d76043bfd92c685601c79189393fc76e03c5214eed32b"),
                parse_scalar("0x0fbcb5720b463bf7e2ccabf373e77d2c10d27e6549f34cfa33eb2d06ea8b900a"),
                parse_scalar("0x26e03abfcc62da0101516b07aede8bc676a10c47299a57bedc6d9fe80484f3da"),
            ]
        );
    }

    #[test]
    fn test_permutation_t4_x5() {
        assert_eq!(
            poseidon::permutation::<BlueSkyConfigT4X5, Scalar, 4>([
                from_const(0),
                from_const(1),
                from_const(2),
                from_const(3),
            ]),
            [
                parse_scalar("0x12dde8a4c46760e349670d241e36ca7abacc991233039f8deaf6c58ce2230ef6"),
                parse_scalar("0x61e95d9456e9223b4d7926dabae10009da2b6fb9134ade8405f6ef1424e66aa1"),
                parse_scalar("0x2fcce25ab9efb3e26276f3b3aff1e02cdf82df48ce8d3eadbff900cfe015775b"),
                parse_scalar("0x2580707d57a8c1c0cad368e8d5705ffd96f269d66e1cd6f1433f93a3c66d9bf8"),
            ]
        );
    }

    #[test]
    fn test_permutation_t5_x5() {
        assert_eq!(
            poseidon::permutation::<BlueSkyConfigT5X5, Scalar, 5>([
                from_const(0),
                from_const(1),
                from_const(2),
                from_const(3),
                from_const(4),
            ]),
            [
                parse_scalar("0x14bbfce637c6a0038f076b56a2ac76bc3f6363a3cef7f1d97acaf94a14ef724d"),
                parse_scalar("0x68abac631a3a117f229d137ffe2bf14d6673495aa3ca484864a713b3b6349369"),
                parse_scalar("0x4a49e726e6c3d0848378d6897b8f67f23ce2ef949ed0ca4a1df255925f8e4bb7"),
                parse_scalar("0x521164b1b6acd0b5fe257d580014ff766ae51b2ed3e2c400c9e748ec864a9dd0"),
                parse_scalar("0x0273a96bdf343289e82b28464897a8a5e3bb02a1ce52fd36ed32862315ccd71b"),
            ]
        );
    }

    #[test]
    fn test_permutation_t3_x7() {
        assert_eq!(
            poseidon::permutation::<BlueSkyConfigT3X7, Scalar, 3>([
                from_const(0),
                from_const(1),
                from_const(2),
            ]),
            [
                parse_scalar("0x5611c141645d7f46d4ca24d427645397b3a62cea62ec3c4377fbeb5ccbf1a9fc"),
                parse_scalar("0x2e37066e101aa3029b13c3b4388d45c7c9ad3d82a693041bb9c1610596f9aec5"),
                parse_scalar("0x2355c6243bc14311531ecec111b3c6026d7d409bc44cba95a52cf4e502cfaa46"),
            ]
        );
    }

    #[test]
    fn test_permutation_t4_x7() {
        assert_eq!(
            poseidon::permutation::<BlueSkyConfigT4X7, Scalar, 4>([
                from_const(0),
                from_const(1),
                from_const(2),
                from_const(3),
            ]),
            [
                parse_scalar("0x4097228e1f413388e3e8e1e1bdc24ceba0b2e76c6b5319a083a18e6f2560c121"),
                parse_scalar("0x1ce165adfeafe428e54214dda603b52f17baa234a07c158515dace64307316b6"),
                parse_scalar("0x604b93462d442faff52bde3816434037f909b61342df6ce78b8eae9dac3b2cd9"),
                parse_scalar("0x7cf85c11e1f3f8ff9b595497177df9a5e838e4a75b2b4b0a36eced4b2c035668"),
            ]
        );
    }

    #[test]
    fn test_permutation_t5_x7() {
        assert_eq!(
            poseidon::permutation::<BlueSkyConfigT5X7, Scalar, 5>([
                from_const(0),
                from_const(1),
                from_const(2),
                from_const(3),
                from_const(4),
            ]),
            [
                parse_scalar("0x59c6fe3393d949a7139ac549872852f835cacbed2c489b2250cb62ef9717794a"),
                parse_scalar("0x4ea98a60f99c8f07bb51c2141b17cf34a8eedc031c50fde1243b5a50794ffd48"),
                parse_scalar("0x157015c462c3bd4c5c23b800946416f18bc1dbe261f0eeed123682f5485b334d"),
                parse_scalar("0x39314066e246f5dce5e9f725de3796de9021b0d6fa6eb8f2ef61186c6cc71964"),
                parse_scalar("0x6b07dba17c91f324b4457790bd47ba2d346a1ce72bcd2c03a01d9a7118943d9e"),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t3() {
        assert_eq!(
            hash_t3_x5(DST, [from_const(0), from_const(1)]),
            [
                parse_scalar("0x724a77c875aef0b5b62a82825b7c38e751501d5653c208937fe60b866257aa48"),
                parse_scalar("0x71f55280c85b4fd6aff2a6d458e6cc64f7d9ccc57664abd3b5065e5890add59d"),
            ]
        );
        assert_eq!(
            hash_t3_x5(from_const(2), [from_const(0), from_const(1)]),
            [
                parse_scalar("0x7b68dcd80fa751ee8f2d76043bfd92c685601c79189393fc76e03c5214eed32b"),
                parse_scalar("0x0fbcb5720b463bf7e2ccabf373e77d2c10d27e6549f34cfa33eb2d06ea8b900a"),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t4() {
        assert_eq!(
            hash_t4_x5(DST, [from_const(0), from_const(1), from_const(2)]),
            [
                parse_scalar("0x4b57f51155a5cc7d3ca33bcca134e300bc026559e97d360b075d6d31ec3eeaaa"),
                parse_scalar("0x0a9e4699fae07ccaad2b88ddca21d2340d10182fb0f9943d7dfb8059415c8bfe"),
                parse_scalar("0x4df52ac5ae83ce74babb6003bd6e0ada0781342837fe856b90b58ffdbd0d522e"),
            ]
        );
        assert_eq!(
            hash_t4_x5(from_const(3), [from_const(0), from_const(1), from_const(2)]),
            [
                parse_scalar("0x12dde8a4c46760e349670d241e36ca7abacc991233039f8deaf6c58ce2230ef6"),
                parse_scalar("0x61e95d9456e9223b4d7926dabae10009da2b6fb9134ade8405f6ef1424e66aa1"),
                parse_scalar("0x2fcce25ab9efb3e26276f3b3aff1e02cdf82df48ce8d3eadbff900cfe015775b"),
            ]
        );
    }

    #[test]
    fn test_capacity_dst_t5() {
        assert_eq!(
            hash_t5_x5(
                DST,
                [from_const(0), from_const(1), from_const(2), from_const(3)]
            ),
            [
                parse_scalar("0x056b239e3ecf02ee3ccdd8ef3d9fb0589a8823edc36b41cbe4cc915a410348d4"),
                parse_scalar("0x1187361736f4c47b7d760945a7d5d6abba5bea3e0722c195b8842e6aa2791017"),
                parse_scalar("0x1b91702ab5c3ef67d15a474346c4d34f889752d49c8bb46cf755a958a6c2e653"),
                parse_scalar("0x6ce5efe89630273c76e30055c7921b2558a03ef6d40677268002aef305045e15"),
            ]
        );
        assert_eq!(
            hash_t5_x5(
                from_const(4),
                [from_const(0), from_const(1), from_const(2), from_const(3)]
            ),
            [
                parse_scalar("0x14bbfce637c6a0038f076b56a2ac76bc3f6363a3cef7f1d97acaf94a14ef724d"),
                parse_scalar("0x68abac631a3a117f229d137ffe2bf14d6673495aa3ca484864a713b3b6349369"),
                parse_scalar("0x4a49e726e6c3d0848378d6897b8f67f23ce2ef949ed0ca4a1df255925f8e4bb7"),
                parse_scalar("0x521164b1b6acd0b5fe257d580014ff766ae51b2ed3e2c400c9e748ec864a9dd0"),
            ]
        );
    }

    #[test]
    fn test_hash_t3_x5_1() {
        assert_eq!(
            hash_t3_x5(DST, [from_const(42)]),
            [
                parse_scalar("0x73952c443e4710be4a4c01e20046008b477f0d6fef5d87409cdebc4cdff3490c"),
                parse_scalar("0x05bf595cdacac4f9eba8679b69dcde4eeeca6db242005bf6b923fde28ea88a46"),
            ]
        );
        assert_eq!(
            hash_t3_x5_0(DST, [from_const(42)]),
            parse_scalar("0x73952c443e4710be4a4c01e20046008b477f0d6fef5d87409cdebc4cdff3490c")
        );
        assert_eq!(
            hash_t3_x5(from_const(42), [from_const(42)]),
            [
                parse_scalar("0x53226144fdb06d3cbc52dd62aa32e9660271fb9060e7c0f26850ac218acbd574"),
                parse_scalar("0x5e5238ba369f11eb934c53c5440d096759b2442537cabc34cc3c59f89d056c97"),
            ]
        );
        assert_eq!(
            hash_t3_x5_0(from_const(42), [from_const(42)]),
            parse_scalar("0x53226144fdb06d3cbc52dd62aa32e9660271fb9060e7c0f26850ac218acbd574")
        );
    }

    #[test]
    fn test_hash_t3_x5_2() {
        assert_eq!(
            hash_t3_x5(DST, [from_const(1), from_const(2)]),
            [
                parse_scalar("0x28935bd3eba75f7b2d4f62babbd4e907b1ffcc28f73d1cae33654441a8a84023"),
                parse_scalar("0x339d0e485d8fdfb8c3391182d457fa3e73f043f566af1463ab05e57045122519"),
            ]
        );
        assert_eq!(
            hash_t3_x5_0(DST, [from_const(1), from_const(2)]),
            parse_scalar("0x28935bd3eba75f7b2d4f62babbd4e907b1ffcc28f73d1cae33654441a8a84023")
        );
    }

    #[test]
    fn test_hash_t3_x5_3() {
        assert_eq!(
            hash_t3_x5(DST, [from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x2bfc323795d99f44817eaa143a7db00103ff1eae1bd67ee3ab3f5a1006c7695d"),
                parse_scalar("0x5e7468521c84b23259b813d193017a2b3c7813ce82e94ce4cc74a8c527db0923"),
            ]
        );
        assert_eq!(
            hash_t3_x5_0(DST, [from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x2bfc323795d99f44817eaa143a7db00103ff1eae1bd67ee3ab3f5a1006c7695d")
        );
    }

    #[test]
    fn test_hash_t3_x5_4() {
        assert_eq!(
            hash_t3_x5(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            [
                parse_scalar("0x06ea9f66eddb8f036b0d6201dcf6a8c610b8aca9371e2bfc7fbd1deb1e5bb158"),
                parse_scalar("0x0bc4c477fdeee23bf2f139b12c2ea927d145f298e6204255cbad8461af9150c6"),
            ]
        );
        assert_eq!(
            hash_t3_x5_0(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            parse_scalar("0x06ea9f66eddb8f036b0d6201dcf6a8c610b8aca9371e2bfc7fbd1deb1e5bb158")
        );
    }

    #[test]
    fn test_hash_t3_x5_5() {
        assert_eq!(
            hash_t3_x5(
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
                parse_scalar("0x05ae2c9b2bdbb5a64d4e838bd96b0b4c2366fc6d3cee4309793e01dfd2a589d1"),
                parse_scalar("0x67de663ef4d5db733c68cae13b6bb28aa97d0fc904dccdfa80f4c9fae36f51d0"),
            ]
        );
        assert_eq!(
            hash_t3_x5_0(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14),
                ]
            ),
            parse_scalar("0x05ae2c9b2bdbb5a64d4e838bd96b0b4c2366fc6d3cee4309793e01dfd2a589d1")
        );
    }

    #[test]
    fn test_hash_t4_x5_1() {
        assert_eq!(
            hash_t4_x5(DST, [from_const(42)]),
            [
                parse_scalar("0x2fdb574b84cca8f2c657ea588d8812bafbba305b7a9933728753de0fcf104c40"),
                parse_scalar("0x732f901b286e0f3575ab52e19494406c38f3db3e06169143f4c0369b3ba58ed9"),
                parse_scalar("0x24c8327a61a3bd811e04b11107609bd91b8916ab5cf53fe927edaa27a9e8d5da"),
            ]
        );
        assert_eq!(
            hash_t4_x5_0(DST, [from_const(42)]),
            parse_scalar("0x2fdb574b84cca8f2c657ea588d8812bafbba305b7a9933728753de0fcf104c40")
        );
        assert_eq!(
            hash_t4_x5(from_const(42), [from_const(42)]),
            [
                parse_scalar("0x0b9c010a47a76f4ee4179c898c90e0397e233230dc87ec82bbeef64784ddb3f8"),
                parse_scalar("0x7f24e07dc0c152c1fcaee1812a77efbc14b3759ed5e0d5344fba0cc0db45db51"),
                parse_scalar("0x2791ee068bd81a0b2a9118983f5136a2d5fe9e1c8dde686b358ccb8b102b2e49"),
            ]
        );
        assert_eq!(
            hash_t4_x5_0(from_const(42), [from_const(42)]),
            parse_scalar("0x0b9c010a47a76f4ee4179c898c90e0397e233230dc87ec82bbeef64784ddb3f8")
        );
    }

    #[test]
    fn test_hash_t4_x5_2() {
        assert_eq!(
            hash_t4_x5(DST, [from_const(1), from_const(2)]),
            [
                parse_scalar("0x33eaaa53f69ea75566e04bcb9318f965d5e74b68663bb4a09adfeeae27c752f4"),
                parse_scalar("0x292ad2994473be89dbfec5185888d85924bfa0f64b3be556609bbde3bad4360c"),
                parse_scalar("0x3f972105e69fcceafe6ce580dab417c50a34316d2de43d73a79f861ef55ca87a"),
            ]
        );
        assert_eq!(
            hash_t4_x5_0(DST, [from_const(1), from_const(2)]),
            parse_scalar("0x33eaaa53f69ea75566e04bcb9318f965d5e74b68663bb4a09adfeeae27c752f4")
        );
    }

    #[test]
    fn test_hash_t4_x5_3() {
        assert_eq!(
            hash_t4_x5(DST, [from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x5220b264d93b85d22b4eb5a19c53ebfd08e1702e00dc76de14603165663006ea"),
                parse_scalar("0x664ca128f4f6f225f282a671b522c267389f30f01d858757a1f029941510d8ec"),
                parse_scalar("0x38ef442cd0ce47da5e7fdd912edfc2a95a36409b142fd0f94545267af135bcfa"),
            ]
        );
        assert_eq!(
            hash_t4_x5_0(DST, [from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x5220b264d93b85d22b4eb5a19c53ebfd08e1702e00dc76de14603165663006ea")
        );
    }

    #[test]
    fn test_hash_t4_x5_4() {
        assert_eq!(
            hash_t4_x5(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            [
                parse_scalar("0x6cef40d837aeb6183356cf40d9818bc0ee109c557b17bffd80ab2905e4e2292f"),
                parse_scalar("0x688cf6e7f2aba6c399bc3253ce3827f7a003f8170fe679cbcc2b37e9ba65211e"),
                parse_scalar("0x1d14218c5f5ae32b4fc20b250b52ad8ec96a77627a6c103c8ecf3919290d6239"),
            ]
        );
        assert_eq!(
            hash_t4_x5_0(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            parse_scalar("0x6cef40d837aeb6183356cf40d9818bc0ee109c557b17bffd80ab2905e4e2292f")
        );
    }

    #[test]
    fn test_hash_t4_x5_5() {
        assert_eq!(
            hash_t4_x5(
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
                parse_scalar("0x22c96d13097aa3b4782f9d2580dc2295378f87c85aaed5f47ee3f8b036faf8ee"),
                parse_scalar("0x0bb3130cba6d1aa9cd4ac577dd503905305ce7ccc08d04ec15d9a9700eb747a1"),
                parse_scalar("0x1cc1f59d0c8b31f60c5b10478b28db466bdcdefda0e8da296d96d5529177d621"),
            ]
        );
        assert_eq!(
            hash_t4_x5_0(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14),
                ]
            ),
            parse_scalar("0x22c96d13097aa3b4782f9d2580dc2295378f87c85aaed5f47ee3f8b036faf8ee")
        );
    }

    #[test]
    fn test_hash_t5_x5_1() {
        assert_eq!(
            hash_t5_x5(DST, [from_const(42)]),
            [
                parse_scalar("0x0e486c122818cc4c0b973b71f5c255506efd9b3554fbcdad85e9ff465591510f"),
                parse_scalar("0x349e93755d1ab6dbe9c40fbadbdc2eca80339bcc0a002dcc6bb5b2b902a6b27f"),
                parse_scalar("0x34b4582aa590ef778b3e778cad2338ae69484eb0f5a0b38f62d21ab1552018b0"),
                parse_scalar("0x608b34493001fcb8e18efd5282972b327e7fa46911212f84e225ee3973354763"),
            ]
        );
        assert_eq!(
            hash_t5_x5_0(DST, [from_const(42)]),
            parse_scalar("0x0e486c122818cc4c0b973b71f5c255506efd9b3554fbcdad85e9ff465591510f")
        );
        assert_eq!(
            hash_t5_x5(from_const(42), [from_const(42)]),
            [
                parse_scalar("0x3d19fd45d8700e6f6a4337c6c3c7e417c0fb1d8f91c22f9a7daccbb05d574213"),
                parse_scalar("0x760fe93c659f66e88e01d159bc50c25cf8f68cda382789eceabfda06d8e54ead"),
                parse_scalar("0x625884fee662eec4c147a13b9ef6ebea187783b33719f2ead163acdbfc1cc921"),
                parse_scalar("0x1ea495d9f36e5a98dda75f19d602510b40d89da10704b87a67340572e9584380"),
            ]
        );
        assert_eq!(
            hash_t5_x5_0(from_const(42), [from_const(42)]),
            parse_scalar("0x3d19fd45d8700e6f6a4337c6c3c7e417c0fb1d8f91c22f9a7daccbb05d574213")
        );
    }

    #[test]
    fn test_hash_t5_x5_2() {
        assert_eq!(
            hash_t5_x5(DST, [from_const(1), from_const(2)]),
            [
                parse_scalar("0x164cc0a2efa358c7babb9a9f7c0268d0a8ded7439fd1a3c55595298bedcc9179"),
                parse_scalar("0x23707f67039f69e3d63086a13d1254674b13fe59301faa4c9f3efeb4b9ef8f18"),
                parse_scalar("0x3fad85404e5bc8510f9847b1a61a2ca9c38f280c8155330c352d66f7282684d5"),
                parse_scalar("0x73b9c75f3c4b3070be35f14b8f611d8e78697fad14330aee0d01bd1ccf1a8717"),
            ]
        );
        assert_eq!(
            hash_t5_x5_0(DST, [from_const(1), from_const(2)]),
            parse_scalar("0x164cc0a2efa358c7babb9a9f7c0268d0a8ded7439fd1a3c55595298bedcc9179")
        );
    }

    #[test]
    fn test_hash_t5_x5_3() {
        assert_eq!(
            hash_t5_x5(DST, [from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x52f946d086fe00334329736601e67c55c36f2762ce802023e28ca82d9a61f350"),
                parse_scalar("0x12971f1e2f2cd1f8d73605bdfb6a19ee4d7ebca4b65673ed1a904b98a6a68e56"),
                parse_scalar("0x57a0b338d530e8b021a82c92b0f8682634e23948a7d6327278ebc6c173bd97cd"),
                parse_scalar("0x4bca6252c6127d62844cf0b67c6a44f9aaf19696a6b1cd9397d2a11ac03a631e"),
            ]
        );
        assert_eq!(
            hash_t5_x5_0(DST, [from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x52f946d086fe00334329736601e67c55c36f2762ce802023e28ca82d9a61f350")
        );
    }

    #[test]
    fn test_hash_t5_x5_4() {
        assert_eq!(
            hash_t5_x5(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            [
                parse_scalar("0x336919de514cf8cef1730217cff9851b74bec5cf77bf76ab54e8fb9679964532"),
                parse_scalar("0x0bbdf8b51fc8718bb66a1c8138299204f190f28ba0ca2b40f250ec628626d753"),
                parse_scalar("0x64c9e9e9df4a14862c6ec70d09b4e380fc9d7554f6e715a715bb731f3a7c5b2c"),
                parse_scalar("0x51c865415c3a958ae4b6bac6a53f2782a080466b4a890973ee97b7b854d4c0d1"),
            ]
        );
        assert_eq!(
            hash_t5_x5_0(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            parse_scalar("0x336919de514cf8cef1730217cff9851b74bec5cf77bf76ab54e8fb9679964532")
        );
    }

    #[test]
    fn test_hash_t5_x5_5() {
        assert_eq!(
            hash_t5_x5(
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
                parse_scalar("0x3c5d07725e5921970556d0184fcbbdbc69be1e71df6d262b98d4d560c35ef1a8"),
                parse_scalar("0x1d6ac33542035df08c624616fa4796d50a7b4e820d4e6250a101d0dc68e96aa4"),
                parse_scalar("0x50f67920b6ba331244353eb4b843ec88f47f34f6664174186a6b1a92a849c0f4"),
                parse_scalar("0x02a81c2d222f324952673ac311ac09a53a5f648199e878d45522c1c0a53f1a2e"),
            ]
        );
        assert_eq!(
            hash_t5_x5_0(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14)
                ]
            ),
            parse_scalar("0x3c5d07725e5921970556d0184fcbbdbc69be1e71df6d262b98d4d560c35ef1a8")
        );
    }

    #[test]
    fn test_hash_t3_x7_1() {
        assert_eq!(
            hash_t3_x7(DST, [from_const(42)]),
            [
                parse_scalar("0x75b1910abd7a1e73af69b7df7ea51f0931f5bbc93a107f4f2fc697e538194530"),
                parse_scalar("0x0ab52d5e45ad51cd68cdaa6ecd35b3b90a2157aea4abbf76567240d12e3e8c96"),
            ]
        );
        assert_eq!(
            hash_t3_x7_0(DST, [from_const(42)]),
            parse_scalar("0x75b1910abd7a1e73af69b7df7ea51f0931f5bbc93a107f4f2fc697e538194530")
        );
        assert_eq!(
            hash_t3_x7(from_const(42), [from_const(42)]),
            [
                parse_scalar("0x7eda0a50b971a9493a23f2d01085208dc976fc2a0b993c20e79c0021eefac3de"),
                parse_scalar("0x329ec7365aa82c5e20e93515326b7140eb8f883c85780add3f213e0f6b8a9922"),
            ]
        );
        assert_eq!(
            hash_t3_x7_0(from_const(42), [from_const(42)]),
            parse_scalar("0x7eda0a50b971a9493a23f2d01085208dc976fc2a0b993c20e79c0021eefac3de")
        );
    }

    #[test]
    fn test_hash_t3_x7_2() {
        assert_eq!(
            hash_t3_x7(DST, [from_const(1), from_const(2)]),
            [
                parse_scalar("0x422f37f348169469a9a57fe967406acc1636810293b96502125812d26e79bb4f"),
                parse_scalar("0x57a053438650e448bb35a89d4f0237f1735b5d7fa7de51f621d37d7b51386659"),
            ]
        );
        assert_eq!(
            hash_t3_x7_0(DST, [from_const(1), from_const(2)]),
            parse_scalar("0x422f37f348169469a9a57fe967406acc1636810293b96502125812d26e79bb4f")
        );
    }

    #[test]
    fn test_hash_t3_x7_3() {
        assert_eq!(
            hash_t3_x7(DST, [from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x13faaa6dc7917d4dd5b23033727aedaa3e2d0f919f6040e0ec9d1595de56489c"),
                parse_scalar("0x1d6f7eb68a54d805b6847a6fccfd72b67b1efc6c2b1e93d32fce3d73fdd273c6"),
            ]
        );
        assert_eq!(
            hash_t3_x7_0(DST, [from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x13faaa6dc7917d4dd5b23033727aedaa3e2d0f919f6040e0ec9d1595de56489c")
        );
    }

    #[test]
    fn test_hash_t3_x7_4() {
        assert_eq!(
            hash_t3_x7(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            [
                parse_scalar("0x62e50b8ce4e31c73a161072942d933d687511e446adb30fbaf8d525e5733a7e2"),
                parse_scalar("0x180b1f97b25dfd36649c85aef730b4e9b4ce4909293da83146a0afd0a9493107"),
            ]
        );
        assert_eq!(
            hash_t3_x7_0(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            parse_scalar("0x62e50b8ce4e31c73a161072942d933d687511e446adb30fbaf8d525e5733a7e2")
        );
    }

    #[test]
    fn test_hash_t3_x7_5() {
        assert_eq!(
            hash_t3_x7(
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
                parse_scalar("0x177641ae0fc00e89c2e644e2b21ebf6772e6a167ec484033b812409f9f2440b9"),
                parse_scalar("0x1f720f9bcd75185ba6eb345fce80506ab0a04b8dc988c7dd9a812d8bac872c5d"),
            ]
        );
        assert_eq!(
            hash_t3_x7_0(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14),
                ]
            ),
            parse_scalar("0x177641ae0fc00e89c2e644e2b21ebf6772e6a167ec484033b812409f9f2440b9")
        );
    }

    #[test]
    fn test_hash_t4_x7_1() {
        assert_eq!(
            hash_t4_x7(DST, [from_const(42)]),
            [
                parse_scalar("0x28a5c0eef936e9a46a3a05a423a501ebe43285eac92283a20551c632a350794e"),
                parse_scalar("0x7c9caece93bb19dba88c9785166abfda6220dd8cba64879b5b632303a09fe285"),
                parse_scalar("0x66766abbdcfa22fe7d76238bd5f25cc3cf6343d5ef58c534adcd2f944329ded4"),
            ]
        );
        assert_eq!(
            hash_t4_x7_0(DST, [from_const(42)]),
            parse_scalar("0x28a5c0eef936e9a46a3a05a423a501ebe43285eac92283a20551c632a350794e")
        );
        assert_eq!(
            hash_t4_x7(from_const(42), [from_const(42)]),
            [
                parse_scalar("0x679f695458327adbe7f65672c5e9a5d5789b084caf4d44d0e2eb59bb06d78a4b"),
                parse_scalar("0x3b9cc54a0c0d038569341455653010d0287d1fd5bfe67fdf1651fb1394e741f8"),
                parse_scalar("0x3af2b3ad7dd951179652b2ad9956297cef072719d13db55647432a8c200ad912"),
            ]
        );
        assert_eq!(
            hash_t4_x7_0(from_const(42), [from_const(42)]),
            parse_scalar("0x679f695458327adbe7f65672c5e9a5d5789b084caf4d44d0e2eb59bb06d78a4b")
        );
    }

    #[test]
    fn test_hash_t4_x7_2() {
        assert_eq!(
            hash_t4_x7(DST, [from_const(1), from_const(2)]),
            [
                parse_scalar("0x5bb6e2129558f8bb34826b061deac28e7f765d1a2f47e5377d5eb20a6dc4ca24"),
                parse_scalar("0x520fb0ce2e9e1d2447c0e86d1f4b82c81dab8851f0f933732c6f2ac6ef66a93d"),
                parse_scalar("0x02f554b6a6daff7d919f73c16f9dadc1ea513114c2a17b7b078593a911c69607"),
            ]
        );
        assert_eq!(
            hash_t4_x7_0(DST, [from_const(1), from_const(2)]),
            parse_scalar("0x5bb6e2129558f8bb34826b061deac28e7f765d1a2f47e5377d5eb20a6dc4ca24")
        );
    }

    #[test]
    fn test_hash_t4_x7_3() {
        assert_eq!(
            hash_t4_x7(DST, [from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x365d0b1f5572264c158e3189094d07ceae3326a76f18380f6729ab15baeff89d"),
                parse_scalar("0x3084fad030cfbbd5062bd88f42b905466ce6aa81ec201484e83ee63dac5deb54"),
                parse_scalar("0x1677a86d4483fca40bd5be110487a5aa4f1bee89b5e7b114ff5ec4faa7b9e444"),
            ]
        );
        assert_eq!(
            hash_t4_x7_0(DST, [from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x365d0b1f5572264c158e3189094d07ceae3326a76f18380f6729ab15baeff89d")
        );
    }

    #[test]
    fn test_hash_t4_x7_4() {
        assert_eq!(
            hash_t4_x7(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            [
                parse_scalar("0x4b82ab04777c9320fb871b3f38222f0616d790fad2a65106b8625e65f6048124"),
                parse_scalar("0x08022ee46e6c1222c8233f378511f7e5d33a2e37548eb1cf5718f6ca9d0c5eb0"),
                parse_scalar("0x767169dec8ada8b1967fb2ac30ebdcf5e90faafec03f5e61ecd9ab6d355294ac"),
            ]
        );
        assert_eq!(
            hash_t4_x7_0(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            parse_scalar("0x4b82ab04777c9320fb871b3f38222f0616d790fad2a65106b8625e65f6048124")
        );
    }

    #[test]
    fn test_hash_t4_x7_5() {
        assert_eq!(
            hash_t4_x7(
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
                parse_scalar("0x2710fc391ee93437eebfe7cf2c31ca91a71b208b7861bcb3a6eeba007d09bcc8"),
                parse_scalar("0x41ae1b0a3e66dfe8fbedcb764d02d9bf7b9119a6df386cff0ecde6c691fc0ff3"),
                parse_scalar("0x436d549c2c48f2ccdb5a6943ee7e4b61c6cbb35c831137186c590b6cda0ac1c7"),
            ]
        );
        assert_eq!(
            hash_t4_x7_0(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14),
                ]
            ),
            parse_scalar("0x2710fc391ee93437eebfe7cf2c31ca91a71b208b7861bcb3a6eeba007d09bcc8")
        );
    }

    #[test]
    fn test_hash_t5_x7_1() {
        assert_eq!(
            hash_t5_x7(DST, [from_const(42)]),
            [
                parse_scalar("0x13be82b0c4195ce8f080d3cd6554355af2ab3b16fb45aa41722989ea38362e41"),
                parse_scalar("0x26595ed9fd3ec85523f7471f326ff62d92c6779c4eea94201044f55978fc32bc"),
                parse_scalar("0x7794dbc5edbc9dea0634f7c80555d50f4f4c63512e2e7f005725dc4502fd7dd7"),
                parse_scalar("0x68c000a14a3d49d52f24516e7948facca6edbf5db8e9f03fe43f1686b0092194"),
            ]
        );
        assert_eq!(
            hash_t5_x7_0(DST, [from_const(42)]),
            parse_scalar("0x13be82b0c4195ce8f080d3cd6554355af2ab3b16fb45aa41722989ea38362e41")
        );
        assert_eq!(
            hash_t5_x7(from_const(42), [from_const(42)]),
            [
                parse_scalar("0x5894614368ee3dc03d7dee4c040f1e71003edae1a315eea51ca236061a59bea7"),
                parse_scalar("0x749353c2ecbcf995e2f3f10f453c422338b6fb2201c92e2d4990f3221698fad5"),
                parse_scalar("0x37e7d885bbd118b0116e6737195d83a6be099da76453e074f46af188725d21eb"),
                parse_scalar("0x7d6e5f3ea0d95197986a1c869174f3cc91d1e5d026b6e4182bb4f1c5f743f276"),
            ]
        );
        assert_eq!(
            hash_t5_x7_0(from_const(42), [from_const(42)]),
            parse_scalar("0x5894614368ee3dc03d7dee4c040f1e71003edae1a315eea51ca236061a59bea7")
        );
    }

    #[test]
    fn test_hash_t5_x7_2() {
        assert_eq!(
            hash_t5_x7(DST, [from_const(1), from_const(2)]),
            [
                parse_scalar("0x02f636930dee060a7655088784179b61d3e4a0689ba762e423dceac5cdb2966f"),
                parse_scalar("0x235c3c3ae13232255fb2f4ed25ff0be91f19867adb8dc2107cdaa1b813af062d"),
                parse_scalar("0x22301ed5490d33441c1bc06776a85bf35359fe3ffd00ee41ee9e1d5655bb5e86"),
                parse_scalar("0x6dfa5cd85ccfe4bf2bc304cb32b6e2f199c567d3a3824df724899ebc1fdb0bb2"),
            ]
        );
        assert_eq!(
            hash_t5_x7_0(DST, [from_const(1), from_const(2)]),
            parse_scalar("0x02f636930dee060a7655088784179b61d3e4a0689ba762e423dceac5cdb2966f")
        );
    }

    #[test]
    fn test_hash_t5_x7_3() {
        assert_eq!(
            hash_t5_x7(DST, [from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x373f2f169f9882cfd34ac6e7824304d63935f76242232983ca93cc950d9c368d"),
                parse_scalar("0x3b310c1bf1daeea900c137ea3187c89832ceb4e45822b5f784cf9da1ab71bb1d"),
                parse_scalar("0x16299b2143770880b6d62d4d8167493c3b869550140ce56f62c5ffeac4d8b613"),
                parse_scalar("0x3301bec857bdc123776f8173877a69fcc81c9e04199ad03751b451de4f7d1e39"),
            ]
        );
        assert_eq!(
            hash_t5_x7_0(DST, [from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x373f2f169f9882cfd34ac6e7824304d63935f76242232983ca93cc950d9c368d")
        );
    }

    #[test]
    fn test_hash_t5_x7_4() {
        assert_eq!(
            hash_t5_x7(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            [
                parse_scalar("0x7a8f6faa9462faaa5989486518e1152478b8c7c661fc4e9e8d2fa131ae99336f"),
                parse_scalar("0x39537c31a80f71df1aa34ea5849d36b683978a9dba26a91700dd6108f4e06d05"),
                parse_scalar("0x13db31dba34a17e92078bb6a0f5d24bbf3fa12a60aa3987b010c16c4f1d02c11"),
                parse_scalar("0x424092ae3cebcf2794f70383210239d26f4787b47ff3f2f9088149ff346017d3"),
            ]
        );
        assert_eq!(
            hash_t5_x7_0(
                DST,
                [from_const(6), from_const(7), from_const(8), from_const(9)]
            ),
            parse_scalar("0x7a8f6faa9462faaa5989486518e1152478b8c7c661fc4e9e8d2fa131ae99336f")
        );
    }

    #[test]
    fn test_hash_t5_x7_5() {
        assert_eq!(
            hash_t5_x7(
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
                parse_scalar("0x6a7023d52f7f5354ef44f9f3e53360a1f14d521f46a62b0c0892a55cab610219"),
                parse_scalar("0x17bc7387c5225be2da04d08f5a84faa745307526db756aa96f57167bd16deff2"),
                parse_scalar("0x0bb38fdf2db79491b469ca436a9b437ba766dd4372b2f8bb4ecbdc46a18c201f"),
                parse_scalar("0x139283c550177a878715f7a1fe907e2495b750428869a7b642a3984e39aadd2a"),
            ]
        );
        assert_eq!(
            hash_t5_x7_0(
                DST,
                [
                    from_const(10),
                    from_const(11),
                    from_const(12),
                    from_const(13),
                    from_const(14)
                ]
            ),
            parse_scalar("0x6a7023d52f7f5354ef44f9f3e53360a1f14d521f46a62b0c0892a55cab610219")
        );
    }
}
