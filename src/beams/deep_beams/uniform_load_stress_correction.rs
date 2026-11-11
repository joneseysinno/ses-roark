/// Elasticity correction added to the `Mc/I` extreme-fiber stress of a
/// simply supported rectangular beam under uniform load `w` per unit length
/// on width `width` (Timoshenko-Goodier): `w/(5b)`, tension on the bottom fiber.
pub fn uniform_load_stress_correction(load_per_length: f64, width: f64) -> f64 {
    load_per_length / (5.0 * width)
}
