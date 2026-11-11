use nalgebra::{DMatrix, DVector};

/// Row- and column-equilibrated LU solve of a segmented system.
/// `None` when a row or column is zero, a pivot is negligible, or the
/// solution is not finite.
pub(crate) fn solve_segmented(
    mut matrix: DMatrix<f64>,
    mut rhs: DVector<f64>,
) -> Option<DVector<f64>> {
    let m = matrix.nrows();
    for i in 0..m {
        let scale = matrix.row(i).amax();
        if scale == 0.0 || !scale.is_finite() {
            return None;
        }
        matrix.row_mut(i).unscale_mut(scale);
        rhs[i] /= scale;
    }
    let mut column_scales = vec![0.0; m];
    for (j, column_scale) in column_scales.iter_mut().enumerate() {
        let scale = matrix.column(j).amax();
        if scale == 0.0 || !scale.is_finite() {
            return None;
        }
        matrix.column_mut(j).unscale_mut(scale);
        *column_scale = scale;
    }
    let lu = matrix.lu();
    let pivots = lu.u().diagonal();
    if pivots.amin() <= 1e-13 * pivots.amax() {
        return None;
    }
    let scaled = lu.solve(&rhs)?;
    if scaled.iter().any(|value| !value.is_finite()) {
        return None;
    }
    Some(DVector::from_fn(m, |i, _| scaled[i] / column_scales[i]))
}
