use super::Expected;

/// Propped cantilever: pinned left, fixed right, load `w` at `a`.
pub fn case_1c(w: f64, a: f64, l: f64, ei: f64) -> Expected {
    let b = l - a;
    let ra = w * b * b * (3.0 * l - b) / (2.0 * l.powi(3));
    let theta_a = -(ra * l * l / 2.0 - w * b * b / 2.0) / ei;
    Expected {
        ra: Some(ra),
        ma: None,
        rb: Some(w - ra),
        mb: Some(-w * a * b * (l + a) / (2.0 * l * l)),
        deflections: vec![(a, theta_a * a + ra * a.powi(3) / (6.0 * ei))],
    }
}
