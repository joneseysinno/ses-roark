//! Continuous beams by the three-moment equation (Roark 8.3).
//!
//! Unknowns are the internal moments over the supports. Slope continuity at
//! each interior support, written with per-span simply supported end slopes
//! from the beam engine plus the end-moment and settlement contributions,
//! gives a tridiagonal system. A fixed end adds a zero-slope equation.

mod simple_span;
mod solve;
mod span_solution;
mod support_reactions;

use super::beam::BeamSolution;
use super::end_condition::EndCondition;
use super::load::Load;

/// One span with uniform `EI` and loads in span-local coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub length: f64,
    pub ei: f64,
    pub loads: Vec<Load>,
}

/// Continuous beam over `spans.len() + 1` supports. End supports are
/// `Pinned` or `Fixed`; `settlements` (support deflections, positive up) is
/// empty or has one entry per support.
#[derive(Debug, Clone, PartialEq)]
pub struct ContinuousBeam {
    pub spans: Vec<Span>,
    pub left: EndCondition,
    pub right: EndCondition,
    pub settlements: Vec<f64>,
}

/// Support moments (internal, sagging positive) and per-span solutions.
#[derive(Debug, Clone, PartialEq)]
pub struct ContinuousSolution {
    pub support_moments: Vec<f64>,
    pub spans: Vec<BeamSolution>,
}
