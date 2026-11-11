use super::coefficients::coefficients;
use crate::beams::state::StateMatrix;

pub(crate) fn transfer(ei: f64, force: f64, k: f64, x: f64) -> StateMatrix {
    let u = k * x;
    let [c1, c2, c3, _, _] = coefficients(u);
    let cos = u.cos();
    let x2 = x * x;
    StateMatrix::new(
        1.0,
        x * c1,
        x2 * c2 / ei,
        x2 * x * c3 / ei,
        0.0,
        cos,
        x * c1 / ei,
        x2 * c2 / ei,
        0.0,
        -force * x * c1,
        cos,
        x * c1,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
