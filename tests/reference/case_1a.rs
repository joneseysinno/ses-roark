use super::Expected;

/// Cantilever fixed at the right, load `w` at `a` (distance `b = l − a` from the wall).
pub fn case_1a(w: f64, a: f64, l: f64, ei: f64) -> Expected {
    let b = l - a;
    Expected {
        ra: None,
        ma: None,
        rb: Some(w),
        mb: Some(-w * b),
        deflections: vec![
            (0.0, -w * b * b * (3.0 * l - b) / (6.0 * ei)),
            (a, -w * b.powi(3) / (3.0 * ei)),
        ],
    }
}
