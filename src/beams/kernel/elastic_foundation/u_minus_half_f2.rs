/// `u − F2(u)/2`, by power series for `|u| < 1`.
pub(crate) fn u_minus_half_f2(u: f64) -> f64 {
    if u.abs() >= 1.0 {
        return u - 0.5 * (u.cosh() * u.sin() + u.sinh() * u.cos());
    }
    let u4 = u.powi(4);
    let mut term = u;
    let mut sum = 0.0;
    for j in 1..=8 {
        let n = 4.0 * j as f64;
        term *= -4.0 * u4 / ((n - 2.0) * (n - 1.0) * n * (n + 1.0));
        sum -= term;
    }
    sum
}
