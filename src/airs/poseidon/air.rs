use alloc::vec::Vec;
use core::borrow::Borrow;
use p3_matrix::Matrix;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, PrimeField32};
use p3_mds::MdsPermutation;

use super::columns::PoseidonCols;
use super::round_flags::eval_round_flags;

pub struct PoseidonAir<F, Mds: Sync, const WIDTH: usize, const ALPHA: u64, const N_ROUNDS: usize> {
    half_num_full_rounds: usize,
    num_partial_rounds: usize,
    round_constants: Vec<F>,
    mds: Mds,
}

impl<F, Mds: Sync, const WIDTH: usize, const ALPHA: u64, const N_ROUNDS: usize>
    PoseidonAir<F, Mds, WIDTH, ALPHA, N_ROUNDS>
{
    pub fn new(
        half_num_full_rounds: usize,
        num_partial_rounds: usize,
        round_constants: Vec<F>,
        mds: Mds,
    ) -> Self {
        let num_rounds = 2 * half_num_full_rounds + num_partial_rounds;
        assert_eq!(num_rounds, N_ROUNDS);
        assert_eq!(round_constants.len(), WIDTH * num_rounds);

        Self {
            half_num_full_rounds,
            num_partial_rounds,
            round_constants,
            mds,
        }
    }
}

impl<F1, F2: Sync, Mds: Sync, const WIDTH: usize, const ALPHA: u64, const N_ROUNDS: usize>
    BaseAir<F1> for PoseidonAir<F2, Mds, WIDTH, ALPHA, N_ROUNDS>
{
    fn width(&self) -> usize {
        PoseidonCols::<F2, WIDTH, N_ROUNDS>::num_cols()
    }
}

impl<AB, F, Mds, const WIDTH: usize, const ALPHA: u64, const N_ROUNDS: usize> Air<AB>
    for PoseidonAir<F, Mds, WIDTH, ALPHA, N_ROUNDS>
where
    AB: AirBuilder,
    F: PrimeField32 + Sync,
    Mds: MdsPermutation<AB::Expr, WIDTH> + Sync,
{
    fn eval(&self, builder: &mut AB) {
        let num_rounds = 2 * self.half_num_full_rounds + self.num_partial_rounds;
        assert_eq!(num_rounds, N_ROUNDS);

        eval_round_flags::<AB, WIDTH, N_ROUNDS>(builder);

        let main = builder.main();
        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &PoseidonCols<AB::Var, WIDTH, N_ROUNDS> = (*local).borrow();
        let next: &PoseidonCols<AB::Var, WIDTH, N_ROUNDS> = (*next).borrow();

        // The partial round flag must be 0 or 1.
        builder.assert_bool(local.partial_round);

        // check that round constants are added correctly
        let constants = self.round_constants.clone();
        for i in 0..WIDTH {
            let mut round_constant = AB::Expr::zero();
            for r in 0..num_rounds {
                let this_round = local.round_flags[r];
                let this_round_constant =
                    AB::Expr::from_canonical_u32(constants[r * WIDTH + i].as_canonical_u32());
                round_constant += this_round * this_round_constant;
            }
            let before = local.start_of_round[i];
            let expected = local.after_constants[i];

            builder.assert_eq(expected, before + round_constant);
        }

        // check that sbox layer is correct
        // partial s-box
        let before = local.after_constants[0];
        let expected = local.after_sbox[0];
        let after = before.into().exp_const_u64::<ALPHA>();
        builder.assert_eq(expected, after);

        // full s-box
        let full_round = AB::Expr::one() - local.partial_round;
        for i in 0..WIDTH {
            let before = local.after_constants[i];
            let expected = local.after_sbox[i];
            let after = before.into().exp_const_u64::<ALPHA>();
            builder.when(full_round.clone()).assert_eq(after, expected);
        }

        // check that MDS layer is correct
        let before: [AB::Expr; WIDTH] = local.after_sbox.map(|x| x.into());
        let expected = local.after_mds;
        let after = self.mds.permute(before);
        for i in 0..WIDTH {
            builder.assert_eq(after[i].clone(), expected[i]);
        }

        // check that end of this round matches start of next round
        let final_step = local.round_flags[N_ROUNDS - 1];
        let not_final_step = AB::Expr::one() - final_step;
        for i in 0..WIDTH {
            let end = local.after_mds[i];
            let start = next.start_of_round[i];
            builder
                .when_transition()
                .when(not_final_step.clone())
                .assert_eq(end, start);
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use p3_baby_bear::{BabyBear, DiffusionMatrixBabyBear};
    use p3_challenger::DuplexChallenger;
    use p3_commit::ExtensionMmcs;
    use p3_dft::Radix2DitParallel;
    use p3_field::extension::BinomialExtensionField;
    use p3_field::Field;
    use p3_fri::{FriConfig, TwoAdicFriPcs};
    use p3_mds::coset_mds::CosetMds;
    use p3_merkle_tree::FieldMerkleTreeMmcs;
    use p3_poseidon2::{Poseidon2, Poseidon2ExternalMatrixGeneral};
    use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};
    use p3_uni_stark::{prove, verify, StarkConfig, VerificationError};
    use rand::{random, thread_rng};
    use tracing_forest::util::LevelFilter;
    use tracing_forest::ForestLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{EnvFilter, Registry};

    use super::super::{generation::generate_trace_rows, PoseidonAir};

    const NUM_HASHES: usize = 680;

    const WIDTH: usize = 8;
    const ALPHA: u64 = 7;
    const N_ROUNDS: usize = 30;

    #[test]
    fn test_poseidon_air() -> Result<(), VerificationError> {
        let env_filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy();

        Registry::default()
            .with(env_filter)
            .with(ForestLayer::default())
            .init();

        type Val = BabyBear;
        type Challenge = BinomialExtensionField<Val, 4>;

        type Perm = Poseidon2<Val, Poseidon2ExternalMatrixGeneral, DiffusionMatrixBabyBear, 16, 7>;
        let perm = Perm::new_from_rng_128(
            Poseidon2ExternalMatrixGeneral,
            DiffusionMatrixBabyBear,
            &mut thread_rng(),
        );

        type MyHash = PaddingFreeSponge<Perm, 16, 8, 8>;
        let hash = MyHash::new(perm.clone());

        type MyCompress = TruncatedPermutation<Perm, 2, 8, 16>;
        let compress = MyCompress::new(perm.clone());

        type ValMmcs = FieldMerkleTreeMmcs<
            <Val as Field>::Packing,
            <Val as Field>::Packing,
            MyHash,
            MyCompress,
            8,
        >;
        let val_mmcs = ValMmcs::new(hash, compress);

        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());

        type Dft = Radix2DitParallel;
        let dft = Dft {};

        type Challenger = DuplexChallenger<Val, Perm, 16, 8>;

        let fri_config = FriConfig {
            log_blowup: 4,
            num_queries: 21,
            proof_of_work_bits: 16,
            mmcs: challenge_mmcs,
        };
        type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
        let pcs = Pcs::new(dft, val_mmcs, fri_config);

        type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;
        let config = MyConfig::new(pcs);

        let mut challenger = Challenger::new(perm.clone());

        let half_num_full_rounds = 4;
        let num_partial_rounds = 22;
        let round_constants = (0..N_ROUNDS * WIDTH).map(|_| random()).collect::<Vec<_>>();
        let inputs = (0..NUM_HASHES).map(|_| random()).collect::<Vec<_>>();

        type Mds = CosetMds<Val, WIDTH>;
        let mds = Mds::default();
        let mds_ext: CosetMds<Challenge, WIDTH> = CosetMds {
            fft_twiddles: mds.fft_twiddles.iter().map(|&x| x.into()).collect(),
            ifft_twiddles: mds.ifft_twiddles.iter().map(|&x| x.into()).collect(),
            weights: mds
                .weights
                .iter()
                .map(|&x| x.into())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        };

        let trace = generate_trace_rows::<Val, WIDTH, ALPHA, N_ROUNDS, Mds>(
            inputs,
            half_num_full_rounds,
            num_partial_rounds,
            round_constants.clone(),
            mds.clone(),
        );
        let proof = prove(
            &config,
            &PoseidonAir::<Val, _, WIDTH, ALPHA, N_ROUNDS>::new(
                half_num_full_rounds,
                num_partial_rounds,
                round_constants.clone(),
                mds,
            ),
            &mut challenger,
            trace,
            &vec![],
        );

        let mut challenger = Challenger::new(perm);
        verify(
            &config,
            &PoseidonAir::<Val, _, WIDTH, ALPHA, N_ROUNDS>::new(
                half_num_full_rounds,
                num_partial_rounds,
                round_constants,
                mds_ext,
            ),
            &mut challenger,
            &proof,
            &vec![],
        )
    }
}
