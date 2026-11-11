use super::BimetalLayer;

/// Free curvature of a bimetallic strip heated by `delta_t` (Timoshenko):
/// `κ = 6(α₂ − α₁)ΔT(1 + m)² / (h[3(1 + m)² + (1 + mn)(m² + 1/(mn))])`,
/// `m = t₁/t₂`, `n = E₁/E₂`, `h = t₁ + t₂`, layer 1 on the bottom.
/// The sign follows the crate convention: a top layer that expands more
/// bends the strip concave down (negative curvature).
pub fn bimetal_curvature(bottom: BimetalLayer, top: BimetalLayer, delta_t: f64) -> f64 {
    let m = bottom.thickness / top.thickness;
    let n = bottom.modulus / top.modulus;
    let h = bottom.thickness + top.thickness;
    let numerator = 6.0 * (top.expansion - bottom.expansion) * delta_t * (1.0 + m).powi(2);
    let denominator = h * (3.0 * (1.0 + m).powi(2) + (1.0 + m * n) * (m * m + 1.0 / (m * n)));
    -numerator / denominator
}
