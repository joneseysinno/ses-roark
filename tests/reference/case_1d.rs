use super::Expected;

/// Fixed-fixed, load `w` at `a`.
pub fn case_1d(w: f64, a: f64, l: f64, ei: f64) -> Expected {
    let b = l - a;
    Expected {
        ra: Some(w * b * b * (3.0 * a + b) / l.powi(3)),
        ma: Some(-w * a * b * b / (l * l)),
        rb: Some(w * a * a * (a + 3.0 * b) / l.powi(3)),
        mb: Some(-w * a * a * b / (l * l)),
        deflections: vec![(a, -w * a.powi(3) * b.powi(3) / (3.0 * ei * l.powi(3)))],
    }
}
