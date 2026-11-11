use super::error::BeamError;
use super::kernel::Resolved;

/// Largest `c·L` passed to the segmented solver (`n = 512`).
pub(crate) const MAX_CHARACTERISTIC_EXTENT: f64 = 8.0 * 512.0;

/// How many equal shooting segments the span needs.
///
/// Tension and foundation transfer matrices grow like `cosh(c·x)`. One matrix
/// across the whole span loses the boundary rows once `c·L` is large, so the
/// span is split when `c·L > 8` into `n = ceil(c·L / 8)` segments. Plain,
/// compression, and variable-section kernels stay on the single 4×4 solve.
pub(crate) fn segment_count(kernel: &Resolved, length: f64) -> Result<usize, BeamError> {
    let rate = match kernel {
        Resolved::Tension { k, .. } => *k,
        Resolved::Foundation { beta, .. } => *beta,
        _ => return Ok(1),
    };
    let extent = rate * length;
    if !extent.is_finite() || extent > MAX_CHARACTERISTIC_EXTENT {
        return Err(BeamError::UnsupportedCombination(
            "span is too long relative to its characteristic length; model it as semi-infinite or infinite",
        ));
    }
    if extent <= 8.0 {
        return Ok(1);
    }
    Ok((extent / 8.0).ceil() as usize)
}
