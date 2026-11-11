/// Transverse (anticlastic) curvature `−ν·κ` accompanying longitudinal curvature `κ`.
pub fn anticlastic_curvature(curvature: f64, poisson: f64) -> f64 {
    -poisson * curvature
}
