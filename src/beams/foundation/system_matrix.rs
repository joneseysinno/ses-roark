use crate::beams::state::StateMatrix;

/// State-space matrix `A` with `s' = A·s` for the unloaded foundation beam:
/// `y' = θ`, `θ' = M/EI`, `M' = V`, `V' = −k_o·y`.
pub(crate) fn system_matrix(ei: f64, modulus: f64) -> StateMatrix {
    StateMatrix::new(
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0 / ei,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        -modulus,
        0.0,
        0.0,
        0.0,
    )
}
