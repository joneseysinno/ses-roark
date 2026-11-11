/// `1 − F1(u)`, by power series for `|u| < 1`.
pub(crate) fn one_minus_f1(u: f64) -> f64 {
    if u.abs() >= 1.0 {
        return 1.0 - u.cosh() * u.cos();
    }
    let u4 = u.powi(4);
    let mut term = 1.0;
    let mut sum = 0.0;
    for j in 1..=8 {
        let n = 4.0 * j as f64;
        term *= -4.0 * u4 / ((n - 3.0) * (n - 2.0) * (n - 1.0) * n);
        sum -= term;
    }
    sum
}
