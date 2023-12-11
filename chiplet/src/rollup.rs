use crate::poseidon_chip::PoseidonChip;
use crate::smt_chip::PathChip;

use halo2_gadgets::poseidon::primitives::Spec;
use halo2_proofs::dev::metadata::Region;
use halo2_proofs::{
    // arithmetic::Field,
    circuit::{Chip, Layouter, SimpleFloorPlanner, Value},
    halo2curves::FieldExt,
    plonk::{Circuit, Column, ConstraintSystem, Error, Instance},
    poly::Rotation,
};
use smt::poseidon::FieldHasher;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
struct RollupConfig<const TX_MAX: usize> {
    initial_root: Column<Instance>,
    final_root: Column<Instance>,
    transactions: [(Column<Instance>, Column<Instance>); TX_MAX],
}

struct RollupChip<
    const TX_MAX: usize,
    F: FieldExt,
    S: Spec<F, WIDTH, RATE> + Clone,
    H: FieldHasher<F, 2> + Clone,
    const WIDTH: usize,
    const RATE: usize,
    const N: usize,
> {
    config: RollupConfig<TX_MAX>,
    markle_chip: PathChip<F, S, H, WIDTH, RATE, N>,
    poseidon_chip: PoseidonChip<F, S, WIDTH, RATE, 2>,
    _marker: PhantomData<F>,
}

impl<
        const TX_MAX: usize,
        F: FieldExt,
        S: Spec<F, WIDTH, RATE> + Clone,
        H: FieldHasher<F, 2> + Clone,
        const WIDTH: usize,
        const RATE: usize,
        const N: usize,
    > RollupChip<TX_MAX, F, S, H, WIDTH, RATE, N>
{
    fn markle_chip(&self) -> PathChip<F, S, H, WIDTH, RATE, N> {
        self.markle_chip.clone()
    }

    fn poseidon_chip(&self) -> PoseidonChip<F, S, WIDTH, RATE, 2> {
        self.poseidon_chip.clone()
    }
}

impl<
        const TX_MAX: usize,
        F: FieldExt,
        S: Spec<F, WIDTH, RATE> + Clone,
        H: FieldHasher<F, 2> + Clone,
        const WIDTH: usize,
        const RATE: usize,
        const N: usize,
    > RollupChip<TX_MAX, F, S, H, WIDTH, RATE, N>
{
    fn prove(&self, ctx: &mut Region) {
        let markle_chip = self.markle_chip();
        let poseidon_chip = self.poseidon_chip();
    }
}

impl<
        const TX_MAX: usize,
        F: FieldExt,
        S: Spec<F, WIDTH, RATE> + Clone,
        H: FieldHasher<F, 2> + Clone,
        const WIDTH: usize,
        const RATE: usize,
        const N: usize,
    > Chip<F> for RollupChip<TX_MAX, F, S, H, WIDTH, RATE, N>
{
    type Config = RollupConfig<TX_MAX>;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

struct RollupCircuit<const TX_MAX: usize, F: FieldExt> {
    pub initial_root: Value<F>,
    pub final_root: Value<F>,
    pub transactions: [(Value<F>, Value<F>); TX_MAX],
}

impl<F: FieldExt, const TX_MAX: usize> Default for RollupCircuit<TX_MAX, F> {
    fn default() -> Self {
        Self {
            initial_root: Value::default(),
            final_root: Value::default(),
            transactions: [(Value::default(), Value::default()); TX_MAX],
        }
    }
}

impl<F: FieldExt, const TX_MAX: usize> Circuit<F> for RollupCircuit<TX_MAX, F> {
    type Config = RollupConfig<TX_MAX>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        todo!()
    }

    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error> {
        todo!()
    }
}
