use super::Layer;
use super::flexural_rigidity::flexural_rigidity;
use super::neutral_axis::neutral_axis;

/// Bending stress at height `z` under moment `moment` (sagging positive),
/// tension positive: `σ = −E(z)·M·(z − z_na)/ΣEI`.
pub fn layer_stress(layers: &[Layer], moment: f64, z: f64) -> f64 {
    let axis = neutral_axis(layers);
    let rigidity = flexural_rigidity(layers);
    let mut top = 0.0;
    let mut modulus = layers.last().map_or(0.0, |layer| layer.modulus);
    for layer in layers {
        top += layer.thickness;
        if z <= top {
            modulus = layer.modulus;
            break;
        }
    }
    -modulus * moment * (z - axis) / rigidity
}
