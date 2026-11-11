use super::state::{State, StateMatrix};

/// Row- and column-equilibrated LU solve; `None` when the system is singular
/// to working precision (a mechanism).
pub(crate) fn solve_scaled(mut matrix: StateMatrix, mut rhs: State) -> Option<State> {
    for i in 0..4 {
        let scale = matrix.row(i).amax();
        if scale == 0.0 || !scale.is_finite() {
            return None;
        }
        matrix.row_mut(i).unscale_mut(scale);
        rhs[i] /= scale;
    }
    let mut column_scales = [0.0; 4];
    for (j, column_scale) in column_scales.iter_mut().enumerate() {
        let scale = matrix.column(j).amax();
        if scale == 0.0 {
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
    Some(State::from_fn(|i, _| scaled[i] / column_scales[i]))
}
