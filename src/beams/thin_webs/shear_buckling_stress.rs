use std::f64::consts::PI;

/// Elastic shear buckling stress `k_s·π²E/(12(1 − ν²))·(t/b)²`, `b` the short side.
pub fn shear_buckling_stress(
    coefficient: f64,
    modulus: f64,
    poisson: f64,
    thickness: f64,
    short_side: f64,
) -> f64 {
    coefficient * PI * PI * modulus / (12.0 * (1.0 - poisson * poisson))
        * (thickness / short_side).powi(2)
}
