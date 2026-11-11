//! # roark
//!
//! Public-domain mechanics formulas organized by topic to grow toward the full
//! coverage of *Roark's Formulas for Stress and Strain*. This crate encodes
//! classical theory; it does not reproduce copyrighted book text or tables.
//!
//! Chapter 1 (Introduction) is covered by these crate docs. Remaining chapters
//! map to the modules below. The first implemented engine is [`beams`]
//! (straight-beam flexure via singularity functions).
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

pub mod analytical_methods;
pub mod beams;
pub mod bearing_shear;
pub mod biomechanics;
pub mod bodies_under_stress;
pub mod columns;
pub mod combined_stress;
pub mod composites;
pub mod curved_beams;
pub mod dynamic_temperature;
pub mod elastic_stability;
pub mod experimental_methods;
pub mod fasteners_joints;
pub mod fatigue_fracture;
pub mod flat_plates;
pub mod numerical_methods;
pub mod plane_area;
pub mod shells;
pub mod stress_concentration;
pub mod stress_strain;
pub mod torsion;

pub use beams::{
    Beam, BeamError, BeamSolution, EndCondition, Kernel, Load, Peak, Reactions, Stiffness, bracket,
};
