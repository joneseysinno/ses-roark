/// Plate-strip rigidity `EI/(1 − ν²)` for a beam whose width suppresses
/// anticlastic curvature.
pub fn flexural_rigidity(modulus: f64, inertia: f64, poisson: f64) -> f64 {
    modulus * inertia / (1.0 - poisson * poisson)
}
