/// Effective flange width `ratio · width` for a caller-supplied shear-lag ratio.
pub fn effective_width(width: f64, ratio: f64) -> f64 {
    ratio * width
}
