use super::f_functions::f_functions;
use crate::beams::state::StateMatrix;

pub(crate) fn transfer(ei: f64, beta: f64, x: f64) -> StateMatrix {
    let [f1, f2, f3, f4] = f_functions(beta * x);
    let b = beta;
    let b2 = b * b;
    let b3 = b2 * b;
    StateMatrix::new(
        f1,
        f2 / (2.0 * b),
        f3 / (2.0 * b2 * ei),
        f4 / (4.0 * b3 * ei),
        -b * f4,
        f1,
        f2 / (2.0 * b * ei),
        f3 / (2.0 * b2 * ei),
        -2.0 * b2 * ei * f3,
        -b * ei * f4,
        f1,
        f2 / (2.0 * b),
        -2.0 * b3 * ei * f2,
        -2.0 * b2 * ei * f3,
        -b * f4,
        f1,
    )
}
