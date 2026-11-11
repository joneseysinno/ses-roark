use super::Expected;

/// Guided left end, pinned right end, uniform load `w`.
pub fn case_2f(w: f64, l: f64, ei: f64) -> Expected {
    Expected {
        ra: None,
        ma: Some(w * l * l / 2.0),
        rb: Some(w * l),
        mb: None,
        deflections: vec![(0.0, -5.0 * w * l.powi(4) / (24.0 * ei))],
    }
}
