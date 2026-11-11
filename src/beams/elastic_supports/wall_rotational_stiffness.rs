use std::f64::consts::PI;

/// Rotational stiffness `M/θ` of a rectangular beam (depth `depth`, width
/// `width`) built into a semi-infinite wall of the same material, from
/// O'Donnell's result `θ = 16.67·M / (π·E·b·h²)` as quoted in Roark 8.6.
pub fn wall_rotational_stiffness(modulus: f64, depth: f64, width: f64) -> f64 {
    PI * modulus * width * depth * depth / 16.67
}
