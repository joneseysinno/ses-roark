/// `[c1..c5]` with `c_n(u) = Σ (−1)^m u^(2m) / (2m + n)!`:
/// `sin u / u`, `(1 − cos u)/u²`, `(u − sin u)/u³`, `(u²/2 − 1 + cos u)/u⁴`,
/// `(u³/6 − u + sin u)/u⁵`. Series below `|u| = 1`, closed form above.
pub(crate) fn coefficients(u: f64) -> [f64; 5] {
    if u.abs() < 1.0 {
        let u2 = u * u;
        let mut out = [0.0; 5];
        for (i, value) in out.iter_mut().enumerate() {
            let n = i + 1;
            let mut term = 1.0 / (1..=n).product::<usize>() as f64;
            let mut sum = term;
            for m in 1..12 {
                let a = (2 * m + n) as f64;
                term *= -u2 / ((a - 1.0) * a);
                sum += term;
            }
            *value = sum;
        }
        return out;
    }
    let (s, c) = u.sin_cos();
    let u2 = u * u;
    [
        s / u,
        (1.0 - c) / u2,
        (u - s) / (u2 * u),
        (u2 / 2.0 - 1.0 + c) / (u2 * u2),
        (u2 * u / 6.0 - u + s) / (u2 * u2 * u),
    ]
}
