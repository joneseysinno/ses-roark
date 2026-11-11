/// `[F1, F2, F3, F4]` at `u = βx`; `F4` uses its power series for `|u| < 1`
/// to avoid cancellation.
pub(crate) fn f_functions(u: f64) -> [f64; 4] {
    let (s, c) = u.sin_cos();
    let (sh, ch) = (u.sinh(), u.cosh());
    let f4 = if u.abs() < 1.0 {
        let u4 = u.powi(4);
        let mut term = 2.0 * u.powi(3) / 3.0;
        let mut sum = term;
        for j in 1..=8 {
            let n = 4.0 * j as f64;
            term *= -4.0 * u4 / (n * (n + 1.0) * (n + 2.0) * (n + 3.0));
            sum += term;
        }
        sum
    } else {
        ch * s - sh * c
    };
    [ch * c, ch * s + sh * c, sh * s, f4]
}
