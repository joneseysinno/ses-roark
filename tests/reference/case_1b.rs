use super::Expected;

/// Guided left end, fixed right end, load `w` at `a`.
pub fn case_1b(w: f64, a: f64, l: f64, ei: f64) -> Expected {
    let b = l - a;
    let ma = w * b * b / (2.0 * l);
    Expected {
        ra: None,
        ma: Some(ma),
        rb: Some(w),
        mb: Some(ma - w * b),
        deflections: vec![(0.0, -w * b * b * (3.0 * l - 2.0 * b) / (12.0 * ei))],
    }
}
