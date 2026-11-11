use super::Expected;

/// Simply supported, load `w` at `a`.
pub fn case_1e(w: f64, a: f64, l: f64, ei: f64) -> Expected {
    let b = l - a;
    let left = |x: f64| -w * b * x * (l * l - b * b - x * x) / (6.0 * ei * l);
    Expected {
        ra: Some(w * b / l),
        ma: None,
        rb: Some(w * a / l),
        mb: None,
        deflections: vec![
            (a, -w * a * a * b * b / (3.0 * ei * l)),
            (a / 2.0, left(a / 2.0)),
        ],
    }
}
