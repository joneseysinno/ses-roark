use crate::beams::state::StateMatrix;

/// Inverse of [`system_matrix`](super::system_matrix::system_matrix).
pub(crate) fn system_inverse(ei: f64, modulus: f64) -> StateMatrix {
    StateMatrix::new(
        0.0,
        0.0,
        0.0,
        -1.0 / modulus,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        ei,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
    )
}
