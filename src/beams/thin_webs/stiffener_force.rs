/// Compression in a transverse stiffener, `τ·t·s·tan α`, for stiffener spacing `spacing`.
pub fn stiffener_force(shear_stress: f64, thickness: f64, spacing: f64, alpha: f64) -> f64 {
    shear_stress * thickness * spacing * alpha.tan()
}
