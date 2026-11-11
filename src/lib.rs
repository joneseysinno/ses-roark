//! # roark
//!
//! Public-domain mechanics formulas organized by topic to grow toward the full
//! coverage of *Roark's Formulas for Stress and Strain*. This crate encodes
//! classical theory; it does not reproduce copyrighted book text or tables.
//!
//! Chapter 1 (Introduction) is covered by these crate docs. Each implemented
//! chapter is a top-level module:
//!
//! - [`beams`]: Chapter 8, beams; flexure of straight bars.
//! - [`plane_area`]: Appendix A, properties of a plane area.
//!
//! Further chapters will be added as new modules.
//!
//! ## Sign and unit conventions (beams)
//!
//! Locked for the singularity-function engine; do not revisit casually:
//!
//! - **`x`**: left to right along the member, origin at the left end.
//! - **`y` (deflection)**: positive **up**. A beam sagging under a downward
//!   load reports **negative** `y`.
//! - **`M` (moment)**: positive = compression in the top flange (sagging),
//!   i.e. the usual structural / AISC convention. Consistent with
//!   `M = EI · y″` with no extra sign flip.
//! - **`θ` (slope)**: `dy/dx`, same sign family as `y`.
//! - **Applied loads**: downward point/distributed loads enter as a **positive
//!   magnitude** in [`beams::Load`]; contribution functions apply the −y sign
//!   internally.
//!
//! Many textbook closed forms use **y positive downward**. Negate reference
//! `y` and `θ` before comparing against this crate.

pub mod beams;
pub mod plane_area;

pub use beams::{
    Beam, BeamError, BeamSolution, EndCondition, Kernel, Load, Peak, Reactions, Stiffness, bracket,
};
