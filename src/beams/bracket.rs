//! Macaulay singularity-function primitive.

/// Macaulay bracket ⟨x − a⟩ⁿ: `0` for `x < a`, `(x − a)ⁿ` for `x ≥ a`.
///
/// `n = 0` is the unit step (`1` at `x == a`, not `0⁰`). Negative `n`
/// (concentrated singularities) evaluate to `0`.
pub fn bracket(x: f64, a: f64, n: i32) -> f64 {
    if x < a || n < 0 {
        0.0
    } else if n == 0 {
        1.0
    } else {
        (x - a).powi(n)
    }
}

#[cfg(test)]
mod tests {
    use super::bracket;

    #[test]
    fn step_n0() {
        assert_eq!(bracket(0.5, 1.0, 0), 0.0);
        assert_eq!(bracket(1.0, 1.0, 0), 1.0);
        assert_eq!(bracket(4.0, 1.0, 0), 1.0);
    }

    #[test]
    fn ramp_n1() {
        assert_eq!(bracket(0.0, 1.0, 1), 0.0);
        assert_eq!(bracket(1.0, 1.0, 1), 0.0);
        assert_eq!(bracket(3.5, 1.0, 1), 2.5);
    }

    #[test]
    fn parabola_n2() {
        assert_eq!(bracket(-2.0, 1.0, 2), 0.0);
        assert_eq!(bracket(4.0, 1.0, 2), 9.0);
    }

    #[test]
    fn cubic_n3() {
        assert_eq!(bracket(0.9, 1.0, 3), 0.0);
        assert_eq!(bracket(3.0, 1.0, 3), 8.0);
    }

    #[test]
    fn quartic_n4() {
        assert_eq!(bracket(1.0, 2.0, 4), 0.0);
        assert_eq!(bracket(5.0, 2.0, 4), 81.0);
    }
}
