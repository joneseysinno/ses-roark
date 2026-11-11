use super::Expected;

/// Simply supported, uniform load `w`.
pub fn case_2e(w: f64, l: f64, ei: f64) -> Expected {
    let y = |x: f64| -w * x * (l.powi(3) - 2.0 * l * x * x + x.powi(3)) / (24.0 * ei);
    Expected {
        ra: Some(w * l / 2.0),
        ma: None,
        rb: Some(w * l / 2.0),
        mb: None,
        deflections: vec![
            (l / 2.0, -5.0 * w * l.powi(4) / (384.0 * ei)),
            (l / 4.0, y(l / 4.0)),
        ],
    }
}
