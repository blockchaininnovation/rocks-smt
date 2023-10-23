use std::marker::PhantomData;

use halo2_proofs::{
    circuit::Chip,
    halo2curves::FieldExt,
    plonk::{Advice, Column, ConstraintSystem, Instance},
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct RollupConfig<const TX_MAX: usize> {
    pub advice: [Column<Advice>; TX_MAX],
    pub instance: [Column<Instance>; 2],
}

pub struct RollupChip<F: FieldExt, const TX_MAX: usize> {
    config: RollupConfig<TX_MAX>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const TX_MAX: usize> Chip<F> for RollupChip<F, TX_MAX> {
    type Config = RollupConfig<TX_MAX>;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: FieldExt, const TX_MAX: usize> RollupChip<F, TX_MAX> {
    fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; TX_MAX],
        instance: [Column<Instance>; 2],
    ) -> <Self as Chip<F>>::Config {
        for column in instance {
            meta.enable_equality(column)
        }
        for column in advice {
            meta.enable_equality(column)
        }

        meta.create_gate("update_proof", |meta| {
            let prev = meta.query_instance(instance[0], Rotation::cur());
            let after = meta.query_instance(instance[1], Rotation::next());
            vec![prev * after]
        });

        RollupConfig { advice, instance }
    }
}
