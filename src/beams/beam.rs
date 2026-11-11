//! Beam model and solved-state types.

use super::end_condition::EndCondition;
use super::kernel::{Kernel, Resolved};
use super::load::Load;
use super::state::State;
use super::stiffness::Stiffness;

/// Elastic straight beam: span, rigidity, kernel, two end conditions, loads.
#[derive(Debug, Clone, PartialEq)]
pub struct Beam {
    pub length: f64,
    pub stiffness: Stiffness,
    /// Shear rigidity `κGA` (Roark 8.10). With it, the state slope is the
    /// bending rotation of the section; plain kernel only.
    pub shear_rigidity: Option<f64>,
    pub kernel: Kernel,
    pub left: EndCondition,
    pub right: EndCondition,
    pub loads: Vec<Load>,
}

/// Support reactions (`None` where the end provides no such reaction).
///
/// Forces are positive upward; moments are internal end moments, positive
/// when sagging (compression on top), as in Roark's `M_A`, `M_B`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reactions {
    pub ra: Option<f64>,
    pub ma: Option<f64>,
    pub rb: Option<f64>,
    pub mb: Option<f64>,
}

/// Solved beam: the left-end state plus everything needed to evaluate the
/// state anywhere along the span.
#[derive(Debug, Clone, PartialEq)]
pub struct BeamSolution {
    pub(crate) beam: Beam,
    pub(crate) kernel: Resolved,
    pub(crate) initial: State,
}
