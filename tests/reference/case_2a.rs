use super::Expected;

/// Cantilever fixed at the right, uniform load `w` over the span.
pub fn case_2a(w: f64, l: f64, ei: f64) -> Expected {
    Expected {
        ra: None,
        ma: None,
        rb: Some(w * l),
        mb: Some(-w * l * l / 2.0),
        deflections: vec![(0.0, -w * l.powi(4) / (8.0 * ei))],
    }
}
