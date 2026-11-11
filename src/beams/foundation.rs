//! Beams on elastic foundations (Roark 8.5; Tables 8.3-8.6).
//!
//! Finite beams use [`Kernel::ElasticFoundation`](super::Kernel::ElasticFoundation)
//! on a [`Beam`](super::Beam) (Table 8.5). Infinite and semi-infinite beams
//! (Table 8.6) superpose decaying infinite-beam responses (Hetényi); a
//! semi-infinite beam adds an end-conditioning force and couple at `x = 0`
//! chosen to satisfy its end condition.

mod characteristic;
mod infinite_state_at;
mod load_response;
mod semi_infinite_solve;
mod semi_infinite_state_at;
mod system_inverse;
mod system_matrix;
mod table_8_3_values;
mod table_8_4_denominators;
mod unit_responses;

pub use characteristic::characteristic;
pub use table_8_3_values::table_8_3_values;
pub use table_8_4_denominators::table_8_4_denominators;

use super::end_condition::EndCondition;
use super::load::Load;

/// Infinitely long beam on a Winkler foundation of modulus `modulus`.
#[derive(Debug, Clone, PartialEq)]
pub struct InfiniteBeam {
    pub ei: f64,
    pub modulus: f64,
    pub loads: Vec<Load>,
}

/// Beam on a Winkler foundation occupying `x ≥ 0`, with an end condition at `x = 0`.
#[derive(Debug, Clone, PartialEq)]
pub struct SemiInfiniteBeam {
    pub ei: f64,
    pub modulus: f64,
    pub left: EndCondition,
    pub loads: Vec<Load>,
}

/// Solved semi-infinite beam.
#[derive(Debug, Clone, PartialEq)]
pub struct SemiInfiniteSolution {
    pub(crate) beam: SemiInfiniteBeam,
    pub(crate) end_force: f64,
    pub(crate) end_couple: f64,
}
