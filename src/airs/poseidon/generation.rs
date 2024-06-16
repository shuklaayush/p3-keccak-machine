use alloc::vec;
use alloc::vec::Vec;
use core::iter;
use p3_maybe_rayon::prelude::{IntoParallelIterator, ParallelSliceMut};

use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use p3_mds::MdsPermutation;
use tracing::instrument;

use super::columns::PoseidonCols;

#[instrument(name = "generate Poseidon trace", skip_all)]
pub fn generate_trace_rows<
    F: PrimeField32,
    const WIDTH: usize,
    const ALPHA: u64,
    const N_ROUNDS: usize,
    Mds,
>(
    inputs: Vec<[F; WIDTH]>,
    half_num_full_rounds: usize,
    num_partial_rounds: usize,
    round_constants: Vec<F>,
    mds: Mds,
) -> RowMajorMatrix<F>
where
    Mds: MdsPermutation<F, WIDTH>,
{
    let num_rows = (inputs.len() * N_ROUNDS).next_power_of_two();
    let num_columns = PoseidonCols::<F, WIDTH, N_ROUNDS>::num_cols();

    let mut trace = RowMajorMatrix::new(vec![F::zero(); num_rows * num_columns], num_columns);

    let (prefix, rows, suffix) = unsafe {
        trace
            .values
            .align_to_mut::<PoseidonCols<F, WIDTH, N_ROUNDS>>()
    };
    assert!(prefix.is_empty(), "Alignment should match");
    assert!(suffix.is_empty(), "Alignment should match");
    assert_eq!(rows.len(), num_rows);

    let padded_inputs = inputs
        .into_par_iter()
        .chain(iter::repeat([F::zero(); WIDTH]));
    rows.par_chunks_mut(N_ROUNDS)
        .zip(padded_inputs)
        .for_each(|(row, input)| {
            let mut row_refs = row.iter_mut().collect::<Vec<_>>();
            generate_trace_rows_for_perm::<F, WIDTH, ALPHA, N_ROUNDS, _>(
                &mut row_refs,
                input,
                half_num_full_rounds,
                num_partial_rounds,
                round_constants.clone(),
                mds.clone(),
            );
        });

    trace
}

pub fn generate_trace_rows_for_perm<
    F: PrimeField32,
    const WIDTH: usize,
    const ALPHA: u64,
    const N_ROUNDS: usize,
    Mds,
>(
    rows: &mut [&mut PoseidonCols<F, WIDTH, N_ROUNDS>],
    input: [F; WIDTH],
    half_num_full_rounds: usize,
    num_partial_rounds: usize,
    round_constants: Vec<F>,
    mds: Mds,
) where
    Mds: MdsPermutation<F, WIDTH>,
{
    // Populate the round input for the first round.
    rows[0].start_of_round = input;

    let this_round_constants = round_constants
        .iter()
        .take(WIDTH)
        .copied()
        .collect::<Vec<_>>();
    let is_partial_round = 0 < half_num_full_rounds + num_partial_rounds;
    generate_trace_row_for_round::<F, WIDTH, ALPHA, N_ROUNDS, Mds>(
        rows[0],
        0,
        is_partial_round,
        this_round_constants,
        mds.clone(),
    );

    for round in 1..rows.len() {
        // Copy previous row's output to next row's input.
        for i in 0..WIDTH {
            rows[round].start_of_round[i] = rows[round - 1].after_mds[i];
        }

        let this_round_constants = round_constants
            .iter()
            .skip(round * WIDTH)
            .take(WIDTH)
            .copied()
            .collect::<Vec<_>>();
        let is_partial_round =
            round >= half_num_full_rounds && round < half_num_full_rounds + num_partial_rounds;
        generate_trace_row_for_round::<F, WIDTH, ALPHA, N_ROUNDS, Mds>(
            rows[round],
            round,
            is_partial_round,
            this_round_constants,
            mds.clone(),
        );
    }
}

fn generate_trace_row_for_round<
    F: PrimeField32,
    const WIDTH: usize,
    const ALPHA: u64,
    const N_ROUNDS: usize,
    Mds,
>(
    row: &mut PoseidonCols<F, WIDTH, N_ROUNDS>,
    round: usize,
    is_partial_round: bool,
    this_round_constants: Vec<F>,
    mds: Mds,
) where
    Mds: MdsPermutation<F, WIDTH>,
{
    row.round_flags[round] = F::one();
    row.partial_round = F::from_bool(is_partial_round);

    // Populate after_constants
    for (i, &constant) in this_round_constants.iter().enumerate() {
        row.after_constants[i] = row.start_of_round[i] + constant;
    }

    // Populate after_sbox
    if is_partial_round {
        row.after_sbox[0] = row.after_constants[0].exp_const_u64::<ALPHA>();
    } else {
        for i in 0..WIDTH {
            row.after_sbox[i] = row.after_constants[i].exp_const_u64::<ALPHA>();
        }
    }

    // Populate after_mds
    let mut state = [F::zero(); WIDTH];
    state[..WIDTH].copy_from_slice(&row.after_sbox[..WIDTH]);

    mds.permute_mut(&mut state);

    row.after_mds[..WIDTH].copy_from_slice(&state[..WIDTH]);
}
