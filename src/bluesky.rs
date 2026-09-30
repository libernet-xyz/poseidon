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

/// Poseidon configuration for BlueSky with T=3 and x^5 S-box.
pub type BlueSkyConfigT3X5 = BlueSkyConfigX5<3>;

/// Poseidon configuration for BlueSky with T=4 and x^5 S-box.
pub type BlueSkyConfigT4X5 = BlueSkyConfigX5<4>;

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

/// Poseidon configuration for BlueSky with T=3 and x^7 S-box.
pub type BlueSkyConfigT3X7 = BlueSkyConfigX7<3>;

/// Poseidon configuration for BlueSky with T=4 and x^7 S-box.
pub type BlueSkyConfigT4X7 = BlueSkyConfigX7<4>;

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
}
