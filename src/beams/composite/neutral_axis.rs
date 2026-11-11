use super::Layer;

/// Height of the transformed-section neutral axis above the bottom face.
pub fn neutral_axis(layers: &[Layer]) -> f64 {
    let mut z = 0.0;
    let (mut stiffness, mut moment) = (0.0, 0.0);
    for layer in layers {
        let axial = layer.modulus * layer.width * layer.thickness;
        stiffness += axial;
        moment += axial * (z + layer.thickness / 2.0);
        z += layer.thickness;
    }
    moment / stiffness
}
