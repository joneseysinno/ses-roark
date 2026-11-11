/// Assert `|actual − expected| ≤ rel·|expected| + abs`.
#[track_caller]
pub fn assert_close(actual: f64, expected: f64, rel: f64, abs: f64) {
    let tolerance = rel * expected.abs() + abs;
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual {actual:e} vs expected {expected:e} (difference {:e}, tolerance {tolerance:e})",
        (actual - expected).abs()
    );
}
