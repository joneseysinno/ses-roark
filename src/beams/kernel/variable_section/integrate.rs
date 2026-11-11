use super::gauss_legendre::gauss_legendre;

/// `∫ₐᵇ f`, split at `breakpoints` inside `(a, b)` and into four equal pieces
/// per interval, each integrated with 16-point Gauss-Legendre.
pub(crate) fn integrate(f: impl Fn(f64) -> f64, a: f64, b: f64, breakpoints: &[f64]) -> f64 {
    const PIECES: usize = 4;
    if b <= a {
        return 0.0;
    }
    let (nodes, weights) = gauss_legendre();
    let mut points: Vec<f64> = std::iter::once(a)
        .chain(breakpoints.iter().copied().filter(|&p| p > a && p < b))
        .chain(std::iter::once(b))
        .collect();
    points.sort_by(f64::total_cmp);
    let mut total = 0.0;
    for pair in points.windows(2) {
        let width = (pair[1] - pair[0]) / PIECES as f64;
        for piece in 0..PIECES {
            let half = 0.5 * width;
            let mid = pair[0] + width * piece as f64 + half;
            total += half
                * nodes
                    .iter()
                    .zip(weights)
                    .map(|(t, w)| w * f(mid + half * t))
                    .sum::<f64>();
        }
    }
    total
}
