//! Beams; flexure of straight bars (Roark 9th ed. Chapter 8).
//!
//! The core is an initial-parameter solver: the state `s = [y, θ, M, V]` at
//! any station is the left-end state carried by a kernel transfer matrix plus
//! one load term per applied action,
//!
//! `s(x) = T(x) · s_A + Σ LT(load, x)`.
//!
//! The kernel ([`Kernel`]) selects the basis functions: polynomials for the
//! common case (Table 8.1), Hetényi functions for elastic foundations
//! (Table 8.5), trigonometric / hyperbolic functions for axial compression or
//! tension (Table 8.7), and quadrature for variable sections (Table 8.11).
//! Two boundary equations per end close a 4x4 system for `s_A`.
//!
//! `V` is the vertical shear with `dM/dx = V` for the plain kernel, so
//! `V(0)` is the upward left reaction and `−V(l)` the upward right reaction.
//! End moments are reported as internal bending moments (sagging positive),
//! matching Roark's `M_A`, `M_B`.
//!
//! Validation against textbook closed forms must account for convention
//! mismatch: many printings take **y positive downward**. Negate reference
//! `y` and `θ` from such sources before comparing.

pub mod axial;
pub mod beam;
pub mod bracket;
pub mod catalog;
pub mod composite;
pub mod continuous;
pub mod deep_beams;
pub mod elastic_supports;
pub mod end_condition;
pub mod error;
pub mod flexural_center;
pub mod foundation;
pub mod frames;
pub mod kernel;
pub mod load;
pub mod plastic;
pub mod response_ratios;
pub mod shear_lag;
pub mod state;
pub mod stiffness;
pub mod thin_webs;
pub mod unsymmetric_bending;
pub mod variable_section;
pub mod wide_beams;

mod assemble_system;
mod beam_new;
mod breakpoints;
mod deflection;
mod ei_at;
mod end_equations;
mod error_display;
mod extremum;
mod from_initial_state;
mod max_deflection;
mod max_moment;
mod moment;
mod provides_moment_reaction;
mod provides_vertical_reaction;
mod ratios_between;
mod reactions;
mod shear;
mod slope;
mod solve;
mod solve_scaled;
mod state_at;
mod sum_forces;
mod sum_moments;
mod uniform;
mod uniform_value;
mod validate;

pub use beam::{Beam, BeamSolution, Reactions};
pub use bracket::bracket;
pub use end_condition::EndCondition;
pub use error::BeamError;
pub use kernel::Kernel;
pub use load::Load;
pub use response_ratios::ResponseRatios;
pub use state::{State, StateMatrix};
pub use stiffness::Stiffness;
pub use sum_forces::sum_forces;
pub use sum_moments::sum_moments;
