use super::Expected;

/// Guided left end, pinned right end, load `w` at `a`.
pub fn case_1f(w: f64, a: f64, l: f64, ei: f64) -> Expected {
    let b = l - a;
    Expected {
        ra: None,
        ma: Some(w * b),
        rb: Some(w),
        mb: None,
        deflections: vec![(0.0, -w * b * (3.0 * l * l - b * b) / (6.0 * ei))],
    }
}
