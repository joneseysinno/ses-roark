use super::Expected;

/// Guided left end, fixed right end, uniform load `w`.
pub fn case_2b(w: f64, l: f64, ei: f64) -> Expected {
    Expected {
        ra: None,
        ma: Some(w * l * l / 6.0),
        rb: Some(w * l),
        mb: Some(-w * l * l / 3.0),
        deflections: vec![(0.0, -w * l.powi(4) / (24.0 * ei))],
    }
}
