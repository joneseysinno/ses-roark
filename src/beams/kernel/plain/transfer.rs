use crate::beams::state::StateMatrix;

/// Polynomial transfer matrix. Shear flexibility `f = 1/(κGA)` adds `−f·x`
/// to the deflection response to `V_A` (shear sliding `y' = ψ − f·V`).
pub(crate) fn transfer(ei: f64, shear_flexibility: f64, x: f64) -> StateMatrix {
    let x2 = x * x;
    let x3 = x2 * x;
    StateMatrix::new(
        1.0,
        x,
        x2 / (2.0 * ei),
        x3 / (6.0 * ei) - shear_flexibility * x,
        0.0,
        1.0,
        x / ei,
        x2 / (2.0 * ei),
        0.0,
        0.0,
        1.0,
        x,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
