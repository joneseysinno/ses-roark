/// Shear rigidity `G·A/F` for [`Beam::shear_rigidity`](crate::beams::Beam::shear_rigidity).
pub fn shear_rigidity(shear_modulus: f64, area: f64, form_factor: f64) -> f64 {
    shear_modulus * area / form_factor
}
