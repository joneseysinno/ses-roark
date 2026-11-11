/// `[h1..h5]` with `h_n(u) = Σ u^(2m) / (2m + n)!`:
/// `sinh u / u`, `(cosh u − 1)/u²`, `(sinh u − u)/u³`,
/// `(cosh u − 1 − u²/2)/u⁴`, `(sinh u − u − u³/6)/u⁵`.
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
                term *= u2 / ((a - 1.0) * a);
                sum += term;
            }
            *value = sum;
        }
        return out;
    }
    let (sh, ch) = (u.sinh(), u.cosh());
    let u2 = u * u;
    [
        sh / u,
        (ch - 1.0) / u2,
        (sh - u) / (u2 * u),
        (ch - 1.0 - u2 / 2.0) / (u2 * u2),
        (sh - u - u2 * u / 6.0) / (u2 * u2 * u),
    ]
}
