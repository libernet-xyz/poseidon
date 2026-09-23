use starkom_ff::PrimeField;

/// Poseidon instance configuration trait.
///
/// `T` is the state vector size.
pub trait Config<F: PrimeField, const T: usize> {
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
    /// NOTE: the provided implementation is constant-time even though it uses
    /// [`pow_small_vartime`](`starkom_ff::Field::pow_small_vartime`) because [`PrimeField::ALPHA`]
    /// is constant, so the vartime algorithm will always run in the same amount of time. The
    /// constant-time algorithm would be slower because it would perform unnecessary
    /// multiplications.
    fn sbox(x: F) -> F {
        x.pow_small_vartime(F::ALPHA)
    }

    /// Returns the constants of the ARC layer stored as a flat array, row-first.
    fn get_round_constants() -> &'static [F];

    /// Returns the constants of the MDS matrix stored as a flat array, row-first.
    fn get_mds_matrix() -> &'static [F];
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
pub fn permutation<Cfg: Config<F, T>, F: PrimeField, const T: usize>(mut state: [F; T]) -> [F; T] {
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

/// Generic Poseidon implementation over the prime field `F` with state size `T`, absorption rate
/// `R`, and capacity `C`.
///
/// `T` must be equal to `R+C`.
///
/// `inputs` must not be empty.
///
/// The scalars provided in the `dst` array are used to initialize the capacity elements; you can
/// specify domain separator tags here. All domain separator tags must be fixed and predetermined by
/// the protocol.
///
/// WARNING: this function implicitly pads with zeros up to the next rate boundary, so for example
/// if the rate is 4 the input sequence [1, 2, 3] will trivially collide with [1, 2, 3, 0]. That is
/// sometimes okay for hashing messages whose length is fixed and determined by the protocol, but
/// otherwise you need to manually prepend a scalar containing the length of the sequence. Example:
///
/// ```ignore
/// const DST: [Scalar] = [from_const(42)];
/// let message = [from_const(12), from_const(34), from_const(56), from_const(78), from_const(90)];
/// let [hash, _, _, _] = poseidon::hash::<poseidon::BlueSkyConfig4, Scalar, 4, 3, 1>(
///     DST, std::iter::once(message.len().into()).chain(message));
/// ```
pub fn hash<Cfg: Config<F, T>, F: PrimeField, const T: usize, const R: usize, const C: usize>(
    dst: [F; C],
    inputs: impl IntoIterator<Item = F>,
) -> [F; R] {
    const { assert!(T == R + C) };
    let mut state = [F::ZERO; T];
    state[(T - C)..T].copy_from_slice(&dst);
    let mut inputs = inputs.into_iter().peekable();
    assert!(inputs.peek().is_some(), "cannot hash an empty sequence");
    while inputs.peek().is_some() {
        for i in 0..R {
            match inputs.next() {
                Some(value) => state[i] += value,
                None => break,
            }
        }
        state = permutation::<Cfg, F, T>(state);
    }
    std::array::from_fn(|i| state[i])
}

/// Convenience function for [hashing](`hash`) with Poseidon and squeezing the first element.
pub fn hash0<Cfg: Config<F, T>, F: PrimeField, const T: usize, const R: usize, const C: usize>(
    dst: [F; C],
    inputs: impl IntoIterator<Item = F>,
) -> F {
    hash::<Cfg, F, T, R, C>(dst, inputs)[0]
}
