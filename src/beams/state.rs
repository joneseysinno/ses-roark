//! Beam state vector `[y, θ, M, V]` and the 4x4 transfer matrix acting on it.

use nalgebra::{Matrix4, RowVector4, Vector4};

/// `[y, θ, M, V]`: deflection (up), slope, bending moment (sagging), vertical shear.
pub type State = Vector4<f64>;

/// Linear map between states: rows `[y, θ, M, V]` at `x`, columns the left-end state.
pub type StateMatrix = Matrix4<f64>;

/// One linear boundary equation `row · s = 0`.
pub(crate) type StateRow = RowVector4<f64>;

pub const DEFLECTION: usize = 0;
pub const SLOPE: usize = 1;
pub const MOMENT: usize = 2;
pub const SHEAR: usize = 3;
