//! Beams under simultaneous axial and transverse loading (Roark 8.7;
//! Tables 8.7-8.10).
//!
//! Table 8.7 cases are a [`Beam`](super::Beam) with
//! [`Kernel::AxialCompression`](super::Kernel::AxialCompression) or
//! [`Kernel::AxialTension`](super::Kernel::AxialTension). This module adds the
//! Table 8.8 functions, the elastic critical load, ends restrained against
//! horizontal displacement (Table 8.9), and response coefficients relative to
//! the unloaded-axially beam (Table 8.10).

mod amplification;
mod critical_load;
mod first_critical_below;
mod restrained_ends;
mod table_8_8_values;

pub use amplification::amplification;
pub use critical_load::critical_load;
pub use first_critical_below::first_critical_below;
pub use restrained_ends::restrained_ends;
pub use table_8_8_values::table_8_8_values;

use super::beam::BeamSolution;

/// Sense of the end load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxialSense {
    Compression,
    Tension,
}

/// Beam with horizontally restrained ends: the induced tension and the
/// corresponding beam solution.
#[derive(Debug, Clone, PartialEq)]
pub struct RestrainedSolution {
    pub tension: f64,
    pub solution: BeamSolution,
}
