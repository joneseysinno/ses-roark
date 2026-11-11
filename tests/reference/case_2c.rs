use super::Expected;

/// Propped cantilever (pinned left, fixed right), uniform load `w`.
pub fn case_2c(w: f64, l: f64, ei: f64) -> Expected {
    Expected {
        ra: Some(3.0 * w * l / 8.0),
        ma: None,
        rb: Some(5.0 * w * l / 8.0),
        mb: Some(-w * l * l / 8.0),
        deflections: vec![(l / 2.0, -w * l.powi(4) / (192.0 * ei))],
    }
}
