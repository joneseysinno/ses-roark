/// Diagonal tension in a fully developed tension field, `2τ / sin 2α`.
pub fn diagonal_tension_stress(shear_stress: f64, alpha: f64) -> f64 {
    2.0 * shear_stress / (2.0 * alpha).sin()
}
