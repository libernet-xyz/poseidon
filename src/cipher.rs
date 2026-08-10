use crate::poseidon::{Config, permutation};
use primitive_types::H512;
use sha3::{self, Digest};
use starkom_ff::PrimeField256;
use std::marker::PhantomData;

fn iv_element<F: PrimeField256>(index: usize) -> F {
    let mut hasher = sha3::Sha3_512::new();
    hasher.update(format!("starkom/poseidon/cipher/{}", index).as_bytes());
    F::from_h512(H512::from_slice(hasher.finalize().as_slice()))
}

#[derive(Debug, Copy, Clone)]
pub struct Encrypt<C: Config<F, T>, F: PrimeField256, const T: usize, const R: usize> {
    state: [F; T],
    _data: PhantomData<C>,
}

impl<C: Config<F, T>, F: PrimeField256, const T: usize, const R: usize> Encrypt<C, F, T, R> {
    pub fn new(key: F) -> Self {
        assert_eq!(R, T - 1);
        let mut state = [F::ZERO; T];
        for i in 0..R {
            state[i] = iv_element(i);
        }
        state[T - 1] = key;
        Self {
            state,
            _data: PhantomData::default(),
        }
    }

    pub fn encrypt(&mut self, block: [F; R]) -> [F; R] {
        self.state = permutation::<C, F, T>(self.state);
        for i in 0..R {
            self.state[i] += block[i];
        }
        std::array::from_fn(|i| self.state[i])
    }

    pub fn finalize(mut self) -> F {
        self.state = permutation::<C, F, T>(self.state);
        self.state[T - 1]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Decrypt<C: Config<F, T>, F: PrimeField256, const T: usize, const R: usize> {
    state: [F; T],
    _data: PhantomData<C>,
}

impl<C: Config<F, T>, F: PrimeField256, const T: usize, const R: usize> Decrypt<C, F, T, R> {
    pub fn new(key: F) -> Self {
        assert_eq!(R, T - 1);
        let mut state = [F::ZERO; T];
        for i in 0..R {
            state[i] = iv_element(i);
        }
        state[T - 1] = key;
        Self {
            state,
            _data: PhantomData::default(),
        }
    }

    pub fn decrypt(&mut self, mut block: [F; R]) -> [F; R] {
        self.state = permutation::<C, F, T>(self.state);
        for i in 0..R {
            let key = self.state[i];
            self.state[i] = block[i];
            block[i] -= key;
        }
        block
    }

    pub fn check(mut self, checksum: F) -> bool {
        self.state = permutation::<C, F, T>(self.state);
        self.state[T - 1] == checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bluesky::BlueSkyConfig3;
    use starkom_bluesky::{Scalar, from_const, parse_scalar};

    fn key1() -> Scalar {
        parse_scalar("0x1a06314aa2caec8bb0b56bee3c47cf459318e72181320ac9d1f3199c1704b236")
    }

    fn key2() -> Scalar {
        parse_scalar("0x02084699c3ba63bf94afa8d0830338aa8c16087f8587517d29748744a6606101")
    }

    #[test]
    fn test_encrypt_one_block_t3_key1() {
        let mut encrypt = Encrypt::<BlueSkyConfig3, Scalar, 3, 2>::new(key1());
        let block = encrypt.encrypt([from_const(12), from_const(34)]);
        let checksum = encrypt.finalize();
        assert_eq!(
            block,
            [
                parse_scalar("0x58535d9773cc328d7171b581df847f73b527701dacee410caf0b8cd285c0a068"),
                parse_scalar("0x7304e0ab500528a114c7f677541778bf12fe402c8eb7fb066d6adaf5f0ab13a5")
            ]
        );
        assert_eq!(
            checksum,
            parse_scalar("0x33e0f86c12c6e6cc82796bf80babf506018dd3e065e42ef27c0452fc8dda3908")
        );
    }

    #[test]
    fn test_encrypt_one_block_t3_key2() {
        let mut encrypt = Encrypt::<BlueSkyConfig3, Scalar, 3, 2>::new(key2());
        let block = encrypt.encrypt([from_const(12), from_const(34)]);
        let checksum = encrypt.finalize();
        assert_eq!(
            block,
            [
                parse_scalar("0x46974bbed8470bcc9de31628f46998e63a44605fbf9be8fbc386e82bb87cfd5a"),
                parse_scalar("0x0d197a8b8acccf87a519cc4825a23acb170b3a4f90bb0605335a6a54cb30a262")
            ]
        );
        assert_eq!(
            checksum,
            parse_scalar("0x42a1d65d0e78b32287597684d57a289e3074de14b3dce774ae333b648cb65b7b")
        );
    }

    // TODO

    #[test]
    fn test_decrypt_one_block_t3_key1() {
        let key = key1();
        let mut encrypt = Encrypt::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let ciphertext = encrypt.encrypt([from_const(12), from_const(34)]);
        let checksum = encrypt.finalize();
        let mut decrypt = Decrypt::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let plaintext = decrypt.decrypt(ciphertext);
        assert!(decrypt.check(checksum));
        assert_eq!(plaintext, [from_const(12), from_const(34)]);
    }

    #[test]
    fn test_decrypt_one_block_t3_key2() {
        let key = key2();
        let mut encrypt = Encrypt::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let ciphertext = encrypt.encrypt([from_const(12), from_const(34)]);
        let checksum = encrypt.finalize();
        let mut decrypt = Decrypt::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let plaintext = decrypt.decrypt(ciphertext);
        assert!(decrypt.check(checksum));
        assert_eq!(plaintext, [from_const(12), from_const(34)]);
    }

    // TODO
}
