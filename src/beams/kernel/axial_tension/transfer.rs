use super::coefficients::coefficients;
use crate::beams::state::StateMatrix;

pub(crate) fn transfer(ei: f64, force: f64, k: f64, x: f64) -> StateMatrix {
    let u = k * x;
    let [h1, h2, h3, _, _] = coefficients(u);
    let cosh = u.cosh();
    let x2 = x * x;
    StateMatrix::new(
        1.0,
        x * h1,
        x2 * h2 / ei,
        x2 * x * h3 / ei,
        0.0,
        cosh,
        x * h1 / ei,
        x2 * h2 / ei,
        0.0,
        force * x * h1,
        cosh,
        x * h1,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
