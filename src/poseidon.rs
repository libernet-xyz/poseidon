use starkom_ff::PrimeField;

/// Poseidon instance configuration trait.
///
/// `R` is the absorption rate and `C` is the capacity; the state size `T` must be equal to `R+C`.
pub trait Config<F: PrimeField, const T: usize, const R: usize, const C: usize> {
    /// Returns the number of full rounds on each side.
    fn num_full_rounds() -> usize;

    /// Returns the number of partial rounds.
    fn num_partial_rounds() -> usize;

    /// Returns the total number of rounds.
    fn num_total_rounds() -> usize {
        Self::num_full_rounds() * 2 + Self::num_partial_rounds()
    }

    /// Applies an optimal S-box for this field.
    ///
    /// For BLS12-381 and BlueSky the S-Box is x^5.
    fn sbox(x: F) -> F;

    /// Returns the constants of the ARC layer stored as a flat array, row-first.
    fn get_round_constants() -> &'static [F];

    /// Returns the constants of the MDS matrix stored as a flat array, row-first.
    fn get_mds_matrix() -> &'static [F];
}

/// Standard x^5 S-box.
///
/// WARNING: this is suitable for BLS12-381 and BlueSky but may not be suitable for other fields.
/// The general requirement is that `F::MAX % 5 != 0`, otherwise this S-box is not a bijection and
/// the resulting Poseidon implementation is unsound.
pub fn sbox5<F: PrimeField>(x: F) -> F {
    x.square().square() * x
}

fn mds<F: PrimeField, const T: usize>(matrix: &[F], state: [F; T]) -> [F; T] {
    let mut result = [F::ZERO; T];
    for i in 0..T {
        for j in 0..T {
            result[i] += matrix[i * T + j] * state[j];
        }
    }
    result
}

/// Runs the Poseidon permutation.
pub fn permutation<
    Cfg: Config<F, T, R, C>,
    F: PrimeField,
    const T: usize,
    const R: usize,
    const C: usize,
>(
    mut state: [F; T],
) -> [F; T] {
    const { assert!(T == R + C) };

    let num_full_rounds = Cfg::num_full_rounds();
    let num_partial_rounds = Cfg::num_partial_rounds();
    let num_total_rounds = Cfg::num_total_rounds();
    assert_eq!(num_total_rounds, 2 * num_full_rounds + num_partial_rounds);

    let c = Cfg::get_round_constants();
    let m = Cfg::get_mds_matrix();

    for r in 0..num_full_rounds {
        for i in 0..T {
            state[i] += c[r * T + i];
        }
        for i in 0..T {
            state[i] = Cfg::sbox(state[i]);
        }
        state = mds::<F, T>(m, state);
    }

    for r in num_full_rounds..(num_full_rounds + num_partial_rounds) {
        for i in 0..T {
            state[i] += c[r * T + i];
        }
        state[0] = Cfg::sbox(state[0]);
        state = mds::<F, T>(m, state);
    }

    for r in (num_full_rounds + num_partial_rounds)..num_total_rounds {
        for i in 0..T {
            state[i] += c[r * T + i];
        }
        for i in 0..T {
            state[i] = Cfg::sbox(state[i]);
        }
        state = mds::<F, T>(m, state);
    }

    state
}

/// Generic Poseidon implementation over the prime field `F` with state size `T`, absoprtion rate
/// `R`, and capacity `C`.
///
/// `inputs` must not be empty.
pub fn hash<
    Cfg: Config<F, T, R, C>,
    F: PrimeField,
    const T: usize,
    const R: usize,
    const C: usize,
>(
    inputs: impl IntoIterator<Item = F>,
) -> [F; R] {
    let mut state = [F::ZERO; T];
    let mut inputs = inputs.into_iter().peekable();
    assert!(inputs.peek().is_some(), "cannot hash an empty sequence");
    while inputs.peek().is_some() {
        for i in 0..(T - C) {
            match inputs.next() {
                Some(value) => state[i] += value,
                None => break,
            }
        }
        state = permutation::<Cfg, F, T, R, C>(state);
    }
    std::array::from_fn(|i| state[i])
}

/// Convenience function for hashing with Poseidon and squeezing the first element.
pub fn hash0<
    Cfg: Config<F, T, R, C>,
    F: PrimeField,
    const T: usize,
    const R: usize,
    const C: usize,
>(
    inputs: impl IntoIterator<Item = F>,
) -> F {
    hash::<Cfg, F, T, R, C>(inputs)[0]
}
