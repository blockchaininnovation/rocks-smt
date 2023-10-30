use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::Field,
    circuit::Layouter,
    circuit::{Chip, SimpleFloorPlanner, Value},
    halo2curves::FieldExt,
    plonk::{Circuit, Column, ConstraintSystem, Error, Instance},
};

#[derive(Clone, Debug)]
struct RollupConfig<const TX_MAX: usize> {
    initial_root: Column<Instance>,
    final_root: Column<Instance>,
    transactions: [(Column<Instance>, Column<Instance>); TX_MAX],
}

struct RollupChip<const TX_MAX: usize, F: Field> {
    config: RollupConfig<TX_MAX>,
    _marker: PhantomData<F>,
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
