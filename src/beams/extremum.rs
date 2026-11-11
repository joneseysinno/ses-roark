use super::peak::Peak;

/// Signed value of largest magnitude of `f` over the segments between
/// consecutive `points`, and the station where it occurs. Each smooth segment
/// is sampled densely (including its left-hand and right-hand limits) and the
/// sample of largest magnitude is refined by golden-section search on `|f|`.
pub(crate) fn extremum(f: impl Fn(f64) -> f64, points: &[f64]) -> Peak {
    const SAMPLES: usize = 64;
    let golden = 0.5 * (5.0_f64.sqrt() - 1.0);
    let pick = |best: Peak, x: f64, value: f64| {
        if value.abs() > best.value.abs() {
            Peak { x, value }
        } else {
            best
        }
    };
    let mut best = Peak { x: 0.0, value: 0.0 };
    for pair in points.windows(2) {
        let (p, q) = (pair[0], pair[1]);
        if q <= p {
            continue;
        }
        let hi = q - (q - p) * 1e-12;
        let xs: Vec<f64> = (0..=SAMPLES)
            .map(|i| p + (hi - p) * i as f64 / SAMPLES as f64)
            .collect();
        let values: Vec<f64> = xs.iter().map(|&x| f(x)).collect();
        let peak = (0..=SAMPLES).fold(0, |arg, i| {
            if values[i].abs() > values[arg].abs() {
                i
            } else {
                arg
            }
        });
        best = pick(best, xs[peak], values[peak]);
        let (mut lo, mut up) = (xs[peak.saturating_sub(1)], xs[(peak + 1).min(SAMPLES)]);
        let mut c = up - golden * (up - lo);
        let mut d = lo + golden * (up - lo);
        let (mut fc, mut fd) = (f(c).abs(), f(d).abs());
        for _ in 0..80 {
            if fc > fd {
                up = d;
                d = c;
                fd = fc;
                c = up - golden * (up - lo);
                fc = f(c).abs();
            } else {
                lo = c;
                c = d;
                fc = fd;
                d = lo + golden * (up - lo);
                fd = f(d).abs();
            }
        }
        let x = 0.5 * (lo + up);
        best = pick(best, x, f(x));
    }
    if let Some(&last) = points.last() {
        best = pick(best, last, f(last));
    }
    best
}
