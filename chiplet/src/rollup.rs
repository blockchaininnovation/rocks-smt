use crate::poseidon_chip::PoseidonChip;
use crate::smt_chip::PathChip;

use halo2_gadgets::poseidon::primitives::Spec;
use halo2_proofs::{
    arithmetic::Field,
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
    F: Field,
    S: Spec<F, WIDTH, RATE>,
    H: FieldHasher<F, 2>,
    const WIDTH: usize,
    const RATE: usize,
    const N: usize,
> {
    config: RollupConfig<TX_MAX>,
    markle_chip: PathChip<F, S, H, WIDTH, RATE, N>,
    _marker: PhantomData<F>,
}

impl<
        const TX_MAX: usize,
        F: FieldExt,
        S: Spec<F, WIDTH, RATE>,
        H: FieldHasher<F, 2>,
        const WIDTH: usize,
        const RATE: usize,
        const N: usize,
    > RollupChip<TX_MAX, F, S, H, WIDTH, RATE, N>
{
    fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn prove(ctx: &mut Region) {}

    fn configure(
        meta: &mut ConstraintSystem<F>,
        initial_root: Column<Instance>,
        final_root: Column<Instance>,
        transactions: [(Column<Instance>, Column<Instance>); TX_MAX],
    ) -> <Self as Chip<F>>::Config {
        meta.enable_equality(initial_root);
        meta.enable_equality(final_root);
        for (account, balance) in transactions {
            meta.enable_equality(account);
            meta.enable_equality(balance);
        }

        meta.create_gate("batch transactions", |meta| {
            let init = meta.query_instance(initial_root, Rotation::cur());
            let fin = meta.query_instance(final_root, Rotation::next());
            vec![F::one()]
        });

        RollupConfig {
            initial_root,
            final_root,
            transactions,
        }
    }
}

impl<const TX_MAX: usize, F: FieldExt> Chip<F> for RollupChip<TX_MAX, F> {
    type Config = RollupConfig<TX_MAX>;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

struct RollupCircuit<const TX_MAX: usize, F: Field> {
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
