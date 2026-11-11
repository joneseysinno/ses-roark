/// Shear buckling coefficient `5.35 + 4(b/a)²` for a panel simply supported
/// on all edges, long side `a ≥` short side `b`.
pub fn shear_buckling_coefficient(long_side: f64, short_side: f64) -> f64 {
    5.35 + 4.0 * (short_side / long_side).powi(2)
}
