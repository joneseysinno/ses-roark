use crate::beams::kernel::variable_section::integrate::integrate;

/// Shear form factor `F = (A/I²)·∫ Q(y)²/b(y) dy` for a section of stacked
/// rectangles `(width, height)` listed bottom to top and centered on a common
/// vertical axis (rectangle → 6/5).
pub fn form_factor_layered(layers: &[(f64, f64)]) -> f64 {
    let area: f64 = layers.iter().map(|(b, h)| b * h).sum();
    let mut bottoms = Vec::with_capacity(layers.len());
    let (mut z, mut first) = (0.0, 0.0);
    for (b, h) in layers {
        bottoms.push(z);
        first += b * h * (z + h / 2.0);
        z += h;
    }
    let centroid = first / area;
    let inertia: f64 = layers
        .iter()
        .zip(&bottoms)
        .map(|((b, h), bottom)| {
            let d = bottom + h / 2.0 - centroid;
            b * h.powi(3) / 12.0 + b * h * d * d
        })
        .sum();
    let mut above = 0.0;
    let mut integral = 0.0;
    for ((b, h), bottom) in layers.iter().zip(&bottoms).rev() {
        let top = bottom + h;
        let q = |y: f64| above + b * ((top - centroid).powi(2) - (y - centroid).powi(2)) / 2.0;
        integral += integrate(|y| q(y).powi(2) / b, *bottom, top, &[]);
        above = q(*bottom);
    }
    area * integral / (inertia * inertia)
}
