/// Peak bending stress `6M/(b_e·t²)` when moment `moment` is carried by a
/// caller-supplied effective width `effective_width` of a plate of thickness `thickness`.
pub fn peak_stress(moment: f64, effective_width: f64, thickness: f64) -> f64 {
    6.0 * moment / (effective_width * thickness * thickness)
}
