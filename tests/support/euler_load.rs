use std::f64::consts::PI;

/// Pinned-pinned Euler load `π²EI/l²`.
pub fn euler_load(ei: f64, length: f64) -> f64 {
    PI * PI * ei / (length * length)
}
