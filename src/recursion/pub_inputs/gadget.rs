//! Instructions for endoscaling public inputs.
use super::primitive::{endoscale, i2lebsp};
use crate::{
    circuit::Layouter,
    plonk::{Advice, Column, Error, Instance, TableColumn},
    poly::Rotation,
};
use pasta_curves::{arithmetic::FieldExt, Fp, Fq};

mod chip;
mod lookup;
use lookup::TableConfig;

/// Instructions to map bitstring public inputs to and from endoscalars.
pub trait PubInputsInstructions<F: FieldExt + PubInputsLookup<K, N>, const K: usize, const N: usize>
{
    /// An N-bit word.
    type Word;

    /// An endoscalar corresponding to an N-bit word.
    type Endoscalar;

    /// Check that a bitstring is consistent with its endoscalar representation.
    ///
    /// These endoscalars are provided as the cells in the public input column.
    fn scalar_check(
        &self,
        layouter: impl Layouter<F>,
        row: usize,
    ) -> Result<(Self::Word, Self::Endoscalar), Error>;
}

/// A trait providing the lookup table for decoding public inputs.
pub trait PubInputsLookup<const K: usize, const N: usize>
where
    Self: std::marker::Sized,
{
    /// A lookup table mapping `K`-bit values to endoscalars.
    fn table() -> [([bool; K], Self); N];
}
