use super::Layer;
use super::neutral_axis::neutral_axis;

/// `Σ E_i (I_i + A_i d_i²)` about the transformed-section neutral axis.
pub fn flexural_rigidity(layers: &[Layer]) -> f64 {
    let axis = neutral_axis(layers);
    let mut z = 0.0;
    let mut rigidity = 0.0;
    for layer in layers {
        let d = z + layer.thickness / 2.0 - axis;
        let area = layer.width * layer.thickness;
        rigidity += layer.modulus * (layer.width * layer.thickness.powi(3) / 12.0 + area * d * d);
        z += layer.thickness;
    }
    rigidity
}
