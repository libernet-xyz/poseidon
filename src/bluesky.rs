use crate::params::decode_constants;
use crate::poseidon::{self, sbox5};
use starkom_bluesky::Scalar;
use std::sync::LazyLock;

/// Poseidon configuration for the BlueSky field.
pub struct BlueSkyConfig<const T: usize, const R: usize, const C: usize> {}

impl poseidon::Config<Scalar, 3, 2, 1> for BlueSkyConfig<3, 2, 1> {
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
            let bytes = include_bytes!("../params/bluesky/arc_t3.bin");
            decode_constants::<Scalar, 192>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 9]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/mds_t3.bin");
            decode_constants::<Scalar, 9>(bytes)
        });
        &*MATRIX
    }
}

impl poseidon::Config<Scalar, 4, 3, 1> for BlueSkyConfig<4, 3, 1> {
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
            let bytes = include_bytes!("../params/bluesky/arc_t4.bin");
            decode_constants::<Scalar, 256>(bytes)
        });
        &*ROUND_CONSTANTS
    }

    fn get_mds_matrix() -> &'static [Scalar] {
        static MATRIX: LazyLock<[Scalar; 16]> = LazyLock::new(|| {
            let bytes = include_bytes!("../params/bluesky/mds_t4.bin");
            decode_constants::<Scalar, 16>(bytes)
        });
        &*MATRIX
    }
}

/// Poseidon configuration for BlueSky with T=3.
pub type BlueSkyConfig3 = BlueSkyConfig<3, 2, 1>;

/// Poseidon configuration for BlueSky with T=4.
pub type BlueSkyConfig4 = BlueSkyConfig<4, 3, 1>;

#[cfg(test)]
mod tests {
    use super::*;
    use starkom_bluesky::{from_const, parse_scalar};

    fn hash_t3(inputs: &[Scalar]) -> [Scalar; 3] {
        poseidon::hash::<BlueSkyConfig3, Scalar, 3, 2, 1>(inputs)
    }

    fn hash_t3_0(inputs: &[Scalar]) -> Scalar {
        poseidon::hash0::<BlueSkyConfig3, Scalar, 3, 2, 1>(inputs)
    }

    fn hash_t4(inputs: &[Scalar]) -> [Scalar; 4] {
        poseidon::hash::<BlueSkyConfig4, Scalar, 4, 3, 1>(inputs)
    }

    fn hash_t4_0(inputs: &[Scalar]) -> Scalar {
        poseidon::hash0::<BlueSkyConfig4, Scalar, 4, 3, 1>(inputs)
    }

    #[test]
    fn test_permutation_t3() {
        assert_eq!(
            poseidon::permutation::<BlueSkyConfig3, Scalar, 3, 2, 1>([
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
    fn test_permutation_t4() {
        assert_eq!(
            poseidon::permutation::<BlueSkyConfig4, Scalar, 4, 3, 1>([
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
    fn test_hash_t3_1() {
        assert_eq!(
            hash_t3(&[from_const(42)]),
            [
                parse_scalar("0x73952c443e4710be4a4c01e20046008b477f0d6fef5d87409cdebc4cdff3490c"),
                parse_scalar("0x05bf595cdacac4f9eba8679b69dcde4eeeca6db242005bf6b923fde28ea88a46"),
                parse_scalar("0x55d401b5483d5c70951f30fa5031b3898c43e831d4a413cb4094c8fbbe1aa9de"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[from_const(42)]),
            parse_scalar("0x73952c443e4710be4a4c01e20046008b477f0d6fef5d87409cdebc4cdff3490c")
        );
    }

    #[test]
    fn test_hash_t3_2() {
        assert_eq!(
            hash_t3(&[from_const(1), from_const(2)]),
            [
                parse_scalar("0x28935bd3eba75f7b2d4f62babbd4e907b1ffcc28f73d1cae33654441a8a84023"),
                parse_scalar("0x339d0e485d8fdfb8c3391182d457fa3e73f043f566af1463ab05e57045122519"),
                parse_scalar("0x748c7c2396801d8341dabf06412a27b394cf5214bdc6adb86423d156a8c9ba02"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[from_const(1), from_const(2)]),
            parse_scalar("0x28935bd3eba75f7b2d4f62babbd4e907b1ffcc28f73d1cae33654441a8a84023")
        );
    }

    #[test]
    fn test_hash_t3_3() {
        assert_eq!(
            hash_t3(&[from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x2bfc323795d99f44817eaa143a7db00103ff1eae1bd67ee3ab3f5a1006c7695d"),
                parse_scalar("0x5e7468521c84b23259b813d193017a2b3c7813ce82e94ce4cc74a8c527db0923"),
                parse_scalar("0x674a798a6315099b5b65e5eb5dae5dadd4bacb7a1e2bcee58fc9f8c888cc596a"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x2bfc323795d99f44817eaa143a7db00103ff1eae1bd67ee3ab3f5a1006c7695d")
        );
    }

    #[test]
    fn test_hash_t3_4() {
        assert_eq!(
            hash_t3(&[from_const(6), from_const(7), from_const(8), from_const(9)]),
            [
                parse_scalar("0x06ea9f66eddb8f036b0d6201dcf6a8c610b8aca9371e2bfc7fbd1deb1e5bb158"),
                parse_scalar("0x0bc4c477fdeee23bf2f139b12c2ea927d145f298e6204255cbad8461af9150c6"),
                parse_scalar("0x4cb7444cd642180aeaa40e7ad201d8d1f78d7ee1366400dd80373ebb0ae9a2d0"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[from_const(6), from_const(7), from_const(8), from_const(9)]),
            parse_scalar("0x06ea9f66eddb8f036b0d6201dcf6a8c610b8aca9371e2bfc7fbd1deb1e5bb158")
        );
    }

    #[test]
    fn test_hash_t3_5() {
        assert_eq!(
            hash_t3(&[
                from_const(10),
                from_const(11),
                from_const(12),
                from_const(13),
                from_const(14),
            ]),
            [
                parse_scalar("0x05ae2c9b2bdbb5a64d4e838bd96b0b4c2366fc6d3cee4309793e01dfd2a589d1"),
                parse_scalar("0x67de663ef4d5db733c68cae13b6bb28aa97d0fc904dccdfa80f4c9fae36f51d0"),
                parse_scalar("0x78638cb8d9a1d4d3baf3f9bfe7a6d6d696794a5fe64f08ae5389f55f4455cf63"),
            ]
        );
        assert_eq!(
            hash_t3_0(&[
                from_const(10),
                from_const(11),
                from_const(12),
                from_const(13),
                from_const(14),
            ]),
            parse_scalar("0x05ae2c9b2bdbb5a64d4e838bd96b0b4c2366fc6d3cee4309793e01dfd2a589d1")
        );
    }

    #[test]
    fn test_hash_t4_1() {
        assert_eq!(
            hash_t4(&[from_const(42)]),
            [
                parse_scalar("0x2fdb574b84cca8f2c657ea588d8812bafbba305b7a9933728753de0fcf104c40"),
                parse_scalar("0x732f901b286e0f3575ab52e19494406c38f3db3e06169143f4c0369b3ba58ed9"),
                parse_scalar("0x24c8327a61a3bd811e04b11107609bd91b8916ab5cf53fe927edaa27a9e8d5da"),
                parse_scalar("0x646361f44a66f48386f5b514de940951363a3394f5fb95488b87d26051249bba"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[from_const(42)]),
            parse_scalar("0x2fdb574b84cca8f2c657ea588d8812bafbba305b7a9933728753de0fcf104c40")
        );
    }

    #[test]
    fn test_hash_t4_2() {
        assert_eq!(
            hash_t4(&[from_const(1), from_const(2)]),
            [
                parse_scalar("0x33eaaa53f69ea75566e04bcb9318f965d5e74b68663bb4a09adfeeae27c752f4"),
                parse_scalar("0x292ad2994473be89dbfec5185888d85924bfa0f64b3be556609bbde3bad4360c"),
                parse_scalar("0x3f972105e69fcceafe6ce580dab417c50a34316d2de43d73a79f861ef55ca87a"),
                parse_scalar("0x3d73ecf48e39063538b626bccc64b91586dd420903a68f6d99cab91b2b37dfea"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[from_const(1), from_const(2)]),
            parse_scalar("0x33eaaa53f69ea75566e04bcb9318f965d5e74b68663bb4a09adfeeae27c752f4")
        );
    }

    #[test]
    fn test_hash_t4_3() {
        assert_eq!(
            hash_t4(&[from_const(3), from_const(4), from_const(5)]),
            [
                parse_scalar("0x5220b264d93b85d22b4eb5a19c53ebfd08e1702e00dc76de14603165663006ea"),
                parse_scalar("0x664ca128f4f6f225f282a671b522c267389f30f01d858757a1f029941510d8ec"),
                parse_scalar("0x38ef442cd0ce47da5e7fdd912edfc2a95a36409b142fd0f94545267af135bcfa"),
                parse_scalar("0x5e0348d56101a88221d50aaf5c15264f7fbd6667649c41ed2884ea6c0bb0a613"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[from_const(3), from_const(4), from_const(5)]),
            parse_scalar("0x5220b264d93b85d22b4eb5a19c53ebfd08e1702e00dc76de14603165663006ea")
        );
    }

    #[test]
    fn test_hash_t4_4() {
        assert_eq!(
            hash_t4(&[from_const(6), from_const(7), from_const(8), from_const(9)]),
            [
                parse_scalar("0x6cef40d837aeb6183356cf40d9818bc0ee109c557b17bffd80ab2905e4e2292f"),
                parse_scalar("0x688cf6e7f2aba6c399bc3253ce3827f7a003f8170fe679cbcc2b37e9ba65211e"),
                parse_scalar("0x1d14218c5f5ae32b4fc20b250b52ad8ec96a77627a6c103c8ecf3919290d6239"),
                parse_scalar("0x1c73c75b6f00cabb2ba253363ead5515a9ffb8a6b9296cc6bb1faecc13e4e322"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[from_const(6), from_const(7), from_const(8), from_const(9)]),
            parse_scalar("0x6cef40d837aeb6183356cf40d9818bc0ee109c557b17bffd80ab2905e4e2292f")
        );
    }

    #[test]
    fn test_hash_t4_5() {
        assert_eq!(
            hash_t4(&[
                from_const(10),
                from_const(11),
                from_const(12),
                from_const(13),
                from_const(14),
            ]),
            [
                parse_scalar("0x22c96d13097aa3b4782f9d2580dc2295378f87c85aaed5f47ee3f8b036faf8ee"),
                parse_scalar("0x0bb3130cba6d1aa9cd4ac577dd503905305ce7ccc08d04ec15d9a9700eb747a1"),
                parse_scalar("0x1cc1f59d0c8b31f60c5b10478b28db466bdcdefda0e8da296d96d5529177d621"),
                parse_scalar("0x11986cd24911e4e8529f30918a734c160ef1f408854346d1e9472ab352668753"),
            ]
        );
        assert_eq!(
            hash_t4_0(&[
                from_const(10),
                from_const(11),
                from_const(12),
                from_const(13),
                from_const(14),
            ]),
            parse_scalar("0x22c96d13097aa3b4782f9d2580dc2295378f87c85aaed5f47ee3f8b036faf8ee")
        );
    }
}
