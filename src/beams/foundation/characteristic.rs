/// Foundation characteristic `β = (k_o / 4EI)^¼` (inverse length).
pub fn characteristic(ei: f64, modulus: f64) -> f64 {
    (modulus / (4.0 * ei)).powf(0.25)
}
