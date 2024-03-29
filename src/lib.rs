#![allow(clippy::module_inception)]
#![warn(clippy::infinite_loop)]
#![cfg_attr(all(feature = "c-ffi", feature = "nightly"), feature(allocator_api))]

#[rustfmt::skip]
pub mod atom_info;
pub mod arena;
pub mod core;
pub mod disp;
pub mod empirical;
pub mod graph;
pub mod molecule;
pub mod parse;
pub mod utils;

#[cfg(feature = "rand")]
pub mod rand;

#[cfg(feature = "c-ffi")]
pub mod ffi;

pub mod prelude {
    pub use crate::arena::{Arena, Container, Molecule};
    pub use crate::core::{Atom, Bond, Chirality};
    pub use crate::disp::fmt_as_dot;
    #[cfg(feature = "mol-svg")]
    pub use crate::disp::fmt_as_svg;
    pub use crate::empirical::EmpiricalFormula;
    pub use crate::graph::compact::GraphCompactor;
    pub use crate::molecule::{Molecule as MolTrait, ValueMolecule};
    pub use crate::parse::smiles::SmilesParser;
    pub use crate::{arena, empirical, smiles};
}

#[cfg(test)]
mod tests;
