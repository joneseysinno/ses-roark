//! Beams of variable section (Roark 8.8, Table 8.11) and slotted beams (8.9).
//!
//! A [`Beam`](super::Beam) with [`Stiffness::Piecewise`](super::Stiffness::Piecewise)
//! or [`Stiffness::Tapered`](super::Stiffness::Tapered) on the plain kernel is
//! solved by quadrature. This module adds Table 8.11-style coefficients and
//! slotted-beam helpers.

mod slot_secondary_moments;
mod slotted_stiffness;
mod taper_coefficients;

pub use slot_secondary_moments::slot_secondary_moments;
pub use slotted_stiffness::slotted_stiffness;
pub use taper_coefficients::taper_coefficients;
