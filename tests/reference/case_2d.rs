use super::Expected;

/// Fixed-fixed, uniform load `w`.
pub fn case_2d(w: f64, l: f64, ei: f64) -> Expected {
    Expected {
        ra: Some(w * l / 2.0),
        ma: Some(-w * l * l / 12.0),
        rb: Some(w * l / 2.0),
        mb: Some(-w * l * l / 12.0),
        deflections: vec![(l / 2.0, -w * l.powi(4) / (384.0 * ei))],
    }
}
