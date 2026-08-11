use crate::poseidon::{Config, permutation};
use anyhow::{Result, anyhow};
use primitive_types::H512;
use sha3::{self, Digest};
use starkom_ff::PrimeField256;
use std::marker::PhantomData;

fn iv_element<F: PrimeField256>(index: usize) -> F {
    let mut hasher = sha3::Sha3_512::new();
    hasher.update(format!("starkom/poseidon/cipher/{}", index).as_bytes());
    F::from_h512(H512::from_slice(hasher.finalize().as_slice()))
}

fn get_initial_state<F: PrimeField256, const T: usize, const R: usize>(key: F, nonce: F) -> [F; T] {
    let mut state = [F::ZERO; T];
    for i in 0..R {
        state[i] = iv_element::<F>(i) + nonce;
    }
    state[T - 1] = key;
    state
}

/// Encrypts an arbitrary number of field elements in batches of `R` using the Poseidon permutation
/// with state size `T`.
///
/// `R` must be `T - 1` (the last element is reserved for capacity).
///
/// This symmetric cipher is implemented using the Poseidon PRP as a block cipher in duplex sponge
/// mode, which is similar to CFB. The key lives in the capacity element.
#[derive(Debug)]
pub struct Encryptor<C: Config<F, T>, F: PrimeField256, const T: usize, const R: usize> {
    nonce: F,
    state: [F; T],
    _data: PhantomData<C>,
}

impl<C: Config<F, T>, F: PrimeField256, const T: usize, const R: usize> Encryptor<C, F, T, R> {
    /// Constructs an `Encryptor` with the specified `key` and `nonce`.
    ///
    /// WARNING: NEVER reuse the same (key, nonce) pair to encrypt two or more different messages,
    /// as doing so would leak information about the plaintexts!
    pub fn with_nonce(key: F, nonce: F) -> Self {
        assert_eq!(R, T - 1);
        Self {
            nonce,
            state: get_initial_state::<F, T, R>(key, nonce),
            _data: PhantomData::default(),
        }
    }

    /// Constructs an `Encryptor` with the specified `key` and a securely generated fresh nonce.
    ///
    /// You can retrieve the nonce by calling [`Self::nonce`].
    pub fn new(key: F) -> Self {
        Self::with_nonce(key, F::random_default())
    }

    /// Returns the nonce used by the `Encryptor`.
    ///
    /// This can be transmitted publicly along with the ciphertext and will be needed to construct
    /// the [`Decryptor`].
    pub fn nonce(&self) -> F {
        self.nonce
    }

    /// Encrypts a block of `R` field elements.
    pub fn encrypt(&mut self, block: [F; R]) -> [F; R] {
        self.state = permutation::<C, F, T>(self.state);
        for i in 0..R {
            self.state[i] += block[i];
        }
        std::array::from_fn(|i| self.state[i])
    }

    /// Performs the final checksumming.
    pub fn finalize(mut self) -> F {
        self.state = permutation::<C, F, T>(self.state);
        self.state[T - 1]
    }
}

/// Decrypts an arbitrary number of field elements in batches of `R` using the Poseidon permutation
/// with state size `T`.
///
/// `R` must be `T - 1` (the last element is reserved for capacity).
///
/// This symmetric cipher is implemented using the Poseidon PRP as a block cipher in duplex sponge
/// mode, which is similar to CFB. The key lives in the capacity element.
#[derive(Debug)]
pub struct Decryptor<C: Config<F, T>, F: PrimeField256, const T: usize, const R: usize> {
    state: [F; T],
    _data: PhantomData<C>,
}

impl<C: Config<F, T>, F: PrimeField256, const T: usize, const R: usize> Decryptor<C, F, T, R> {
    /// Constructs a `Decryptor` with the specified `key` and `nonce`.
    pub fn new(key: F, nonce: F) -> Self {
        assert_eq!(R, T - 1);
        Self {
            state: get_initial_state::<F, T, R>(key, nonce),
            _data: PhantomData::default(),
        }
    }

    /// Decrypts a block of `R` field elements.
    pub fn decrypt(&mut self, mut block: [F; R]) -> [F; R] {
        self.state = permutation::<C, F, T>(self.state);
        for i in 0..R {
            let key = self.state[i];
            self.state[i] = block[i];
            block[i] -= key;
        }
        block
    }

    /// Performs the final authentication.
    pub fn finalize(mut self, checksum: F) -> Result<()> {
        self.state = permutation::<C, F, T>(self.state);
        if self.state[T - 1].ct_ne(&checksum).into() {
            return Err(anyhow!("invalid checksum {}", checksum));
        }
        Ok(())
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
        let key = key1();
        let nonce = from_const(42);
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::with_nonce(key, nonce);
        let block = encryptor.encrypt([from_const(12), from_const(34)]);
        let checksum = encryptor.finalize();
        assert_eq!(
            block,
            [
                parse_scalar("0x2307fa34de8cc857511a6ffd5c5a75c2ac280e1590cda33b7d255a278161af22"),
                parse_scalar("0x631ab9ce12321bd66b3a4476558038375dbd5a92866b91b7a8e4202ca61d7dfa")
            ]
        );
        assert_eq!(
            checksum,
            parse_scalar("0x6f9251844ea80a125aac5cb50eae00be052eb6059fa2676e87611994cac4825e")
        );
    }

    #[test]
    fn test_encrypt_one_block_t3_key2() {
        let key = key2();
        let nonce = from_const(42);
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::with_nonce(key, nonce);
        let block = encryptor.encrypt([from_const(12), from_const(34)]);
        let checksum = encryptor.finalize();
        assert_eq!(
            block,
            [
                parse_scalar("0x3fefa9a61ab2d7c6f84934cbb502f612083c3c683b4bbf4d3fd8430f23e39b1a"),
                parse_scalar("0x7f4aa6d73c8cfa472f92d771d0459ce958c230cdacc7dd3164209a9fe7f9d88c")
            ]
        );
        assert_eq!(
            checksum,
            parse_scalar("0x5ccd3bea81baecca7e1a37a2215251fbbba2cc9b18924547ad321ae0286955a3")
        );
    }

    #[test]
    fn test_encrypt_one_block_t3_different_nonces() {
        let key = key1();
        let mut encryptor1 = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let block1 = encryptor1.encrypt([from_const(12), from_const(34)]);
        let checksum1 = encryptor1.finalize();
        let mut encryptor2 = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let block2 = encryptor2.encrypt([from_const(12), from_const(34)]);
        let checksum2 = encryptor2.finalize();
        assert_ne!(block1, block2);
        assert_ne!(checksum1, checksum2);
    }

    #[test]
    fn test_encrypt_two_blocks_t3_key1() {
        let key = key1();
        let nonce = from_const(42);
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::with_nonce(key, nonce);
        let block1 = encryptor.encrypt([from_const(34), from_const(56)]);
        let block2 = encryptor.encrypt([from_const(78), from_const(90)]);
        let checksum = encryptor.finalize();
        assert_eq!(
            block1,
            [
                parse_scalar("0x2307fa34de8cc857511a6ffd5c5a75c2ac280e1590cda33b7d255a278161af38"),
                parse_scalar("0x631ab9ce12321bd66b3a4476558038375dbd5a92866b91b7a8e4202ca61d7e10")
            ]
        );
        assert_eq!(
            block2,
            [
                parse_scalar("0x6432f7934ca848ba66d3a8cc2500e26df40c4d4e8552bc051a352b5036adb848"),
                parse_scalar("0x648dae2392ea9efc70616a33306b3d15f56185c55ab91c68ba01f3e0027f0b0e")
            ]
        );
        assert_eq!(
            checksum,
            parse_scalar("0x093e03fb2f47bfc00585152cad24975804ba18ce4f705aa51928a360a313e40c")
        );
    }

    #[test]
    fn test_encrypt_two_blocks_t3_key2() {
        let key = key2();
        let nonce = from_const(42);
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::with_nonce(key, nonce);
        let block1 = encryptor.encrypt([from_const(34), from_const(56)]);
        let block2 = encryptor.encrypt([from_const(78), from_const(90)]);
        let checksum = encryptor.finalize();
        assert_eq!(
            block1,
            [
                parse_scalar("0x3fefa9a61ab2d7c6f84934cbb502f612083c3c683b4bbf4d3fd8430f23e39b30"),
                parse_scalar("0x7f4aa6d73c8cfa472f92d771d0459ce958c230cdacc7dd3164209a9fe7f9d8a2")
            ]
        );
        assert_eq!(
            block2,
            [
                parse_scalar("0x6c404df8c619bc5bc8751cc66a57fc8fccef7d427b136b9f7f99cc2025f0d830"),
                parse_scalar("0x55ae7196244639cb33aca30d47424baa09d61ee6b878dd250a3e3f871d15d71a")
            ]
        );
        assert_eq!(
            checksum,
            parse_scalar("0x70517f6c982dc4927bc74cb76b461fff2d22e80fe27a14f4131fc75e87ea2fcf")
        );
    }

    #[test]
    fn test_encrypt_two_blocks_t3_different_nonces() {
        let key = key1();
        let mut encryptor1 = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let block11 = encryptor1.encrypt([from_const(34), from_const(56)]);
        let block12 = encryptor1.encrypt([from_const(78), from_const(90)]);
        let checksum1 = encryptor1.finalize();
        let mut encryptor2 = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let block21 = encryptor2.encrypt([from_const(34), from_const(56)]);
        let block22 = encryptor2.encrypt([from_const(78), from_const(90)]);
        let checksum2 = encryptor2.finalize();
        assert_ne!(block11, block21);
        assert_ne!(block12, block22);
        assert_ne!(checksum1, checksum2);
    }

    // TODO

    #[test]
    fn test_decrypt_one_block_t3_key1() {
        let key = key1();
        let nonce = from_const(42);
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::with_nonce(key, nonce);
        let ciphertext = encryptor.encrypt([from_const(12), from_const(34)]);
        let checksum = encryptor.finalize();
        let mut decryptor = Decryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key, nonce);
        let plaintext = decryptor.decrypt(ciphertext);
        assert!(decryptor.finalize(checksum).is_ok());
        assert_eq!(plaintext, [from_const(12), from_const(34)]);
    }

    #[test]
    fn test_decrypt_one_block_t3_key2() {
        let key = key2();
        let nonce = from_const(42);
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::with_nonce(key, nonce);
        let ciphertext = encryptor.encrypt([from_const(12), from_const(34)]);
        let checksum = encryptor.finalize();
        let mut decryptor = Decryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key, nonce);
        let plaintext = decryptor.decrypt(ciphertext);
        assert!(decryptor.finalize(checksum).is_ok());
        assert_eq!(plaintext, [from_const(12), from_const(34)]);
    }

    #[test]
    fn test_decrypt_one_block_t3_automatic_nonce() {
        let key = key1();
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let nonce = encryptor.nonce();
        let ciphertext = encryptor.encrypt([from_const(12), from_const(34)]);
        let checksum = encryptor.finalize();
        let mut decryptor = Decryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key, nonce);
        let plaintext = decryptor.decrypt(ciphertext);
        assert!(decryptor.finalize(checksum).is_ok());
        assert_eq!(plaintext, [from_const(12), from_const(34)]);
    }

    #[test]
    fn test_decrypt_two_blocks_t3_key1() {
        let key = key1();
        let nonce = from_const(42);
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::with_nonce(key, nonce);
        let ciphertext1 = encryptor.encrypt([from_const(34), from_const(56)]);
        let ciphertext2 = encryptor.encrypt([from_const(78), from_const(90)]);
        let checksum = encryptor.finalize();
        let mut decryptor = Decryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key, nonce);
        let plaintext1 = decryptor.decrypt(ciphertext1);
        let plaintext2 = decryptor.decrypt(ciphertext2);
        assert!(decryptor.finalize(checksum).is_ok());
        assert_eq!(plaintext1, [from_const(34), from_const(56)]);
        assert_eq!(plaintext2, [from_const(78), from_const(90)]);
    }

    #[test]
    fn test_decrypt_two_blocks_t3_key2() {
        let key = key2();
        let nonce = from_const(42);
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::with_nonce(key, nonce);
        let ciphertext1 = encryptor.encrypt([from_const(34), from_const(56)]);
        let ciphertext2 = encryptor.encrypt([from_const(78), from_const(90)]);
        let checksum = encryptor.finalize();
        let mut decryptor = Decryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key, nonce);
        let plaintext1 = decryptor.decrypt(ciphertext1);
        let plaintext2 = decryptor.decrypt(ciphertext2);
        assert!(decryptor.finalize(checksum).is_ok());
        assert_eq!(plaintext1, [from_const(34), from_const(56)]);
        assert_eq!(plaintext2, [from_const(78), from_const(90)]);
    }

    #[test]
    fn test_decrypt_two_blocks_t3_automatic_nonce() {
        let key = key1();
        let mut encryptor = Encryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key);
        let nonce = encryptor.nonce();
        let ciphertext1 = encryptor.encrypt([from_const(34), from_const(56)]);
        let ciphertext2 = encryptor.encrypt([from_const(78), from_const(90)]);
        let checksum = encryptor.finalize();
        let mut decryptor = Decryptor::<BlueSkyConfig3, Scalar, 3, 2>::new(key, nonce);
        let plaintext1 = decryptor.decrypt(ciphertext1);
        let plaintext2 = decryptor.decrypt(ciphertext2);
        assert!(decryptor.finalize(checksum).is_ok());
        assert_eq!(plaintext1, [from_const(34), from_const(56)]);
        assert_eq!(plaintext2, [from_const(78), from_const(90)]);
    }

    // TODO
}
