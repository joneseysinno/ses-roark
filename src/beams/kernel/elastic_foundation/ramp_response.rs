use super::f_functions::f_functions;
use super::one_minus_f1::one_minus_f1;
use super::u_minus_half_f2::u_minus_half_f2;
use crate::beams::state::State;

pub(crate) fn ramp_response(modulus: f64, beta: f64, x: f64) -> State {
    let u = beta * x;
    let [_, _, f3, f4] = f_functions(u);
    State::new(
        u_minus_half_f2(u) / (beta * modulus),
        one_minus_f1(u) / modulus,
        f4 / (4.0 * beta.powi(3)),
        f3 / (2.0 * beta * beta),
    )
}
