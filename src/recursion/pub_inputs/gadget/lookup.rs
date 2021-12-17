use super::{endoscale, i2lebsp};
use crate::{
    circuit::Layouter,
    plonk::{ConstraintSystem, Error, TableColumn},
};
use pasta_curves::arithmetic::FieldExt;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct TableConfig<F: FieldExt, const K: usize, const N: usize> {
    pub(in crate::recursion) bits: TableColumn,
    pub(in crate::recursion) endoscalar: TableColumn,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const K: usize, const N: usize> TableConfig<F, K, N> {
    pub fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        TableConfig {
            bits: meta.lookup_table_column(),
            endoscalar: meta.lookup_table_column(),
            _marker: PhantomData,
        }
    }

    pub fn load(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_table(
            || "endoscalar_map",
            |mut table| {
                for index in 0..N {
                    table.assign_cell(
                        || "bits",
                        self.bits,
                        index,
                        || Ok(F::from_u64(index as u64)),
                    )?;
                    table.assign_cell(
                        || "endoscalar",
                        self.endoscalar,
                        index,
                        || Ok(endoscale::<F, K>(i2lebsp(index as u64))),
                    )?;
                }
                Ok(())
            },
        )
    }
}
