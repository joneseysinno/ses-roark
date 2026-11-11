use super::f_functions::f_functions;
use super::one_minus_f1::one_minus_f1;
use crate::beams::state::State;

pub(crate) fn curvature_response(ei: f64, beta: f64, x: f64) -> State {
    let u = beta * x;
    let [_, f2, f3, f4] = f_functions(u);
    State::new(
        f3 / (2.0 * beta * beta),
        f2 / (2.0 * beta),
        -ei * one_minus_f1(u),
        -beta * ei * f4,
    )
}
