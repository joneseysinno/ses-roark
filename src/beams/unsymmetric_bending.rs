//! Beams not loaded in a plane of symmetry (Roark 8.14).
//!
//! Moments are defined by the stresses they resultant from:
//! `M_x = −∫σ·y dA` and `M_y = −∫σ·x dA` with `(x, y)` measured from the
//! centroid and tension positive. A positive `M_x` (sagging about the
//! horizontal axis) compresses the `+y` fibers; a positive `M_y` compresses
//! the `+x` fibers.

mod bending_stress;
mod neutral_axis_angle;

pub use bending_stress::bending_stress;
pub use neutral_axis_angle::neutral_axis_angle;
