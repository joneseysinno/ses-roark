/// Von Kármán effective width `1.9·t·√(E/σ)` of a thin compression flange
/// supported along both edges, at edge stress `max_stress`.
pub fn compression_flange_effective_width(thickness: f64, modulus: f64, max_stress: f64) -> f64 {
    1.9 * thickness * (modulus / max_stress).sqrt()
}
