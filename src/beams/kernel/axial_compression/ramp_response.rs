use super::coefficients::coefficients;
use crate::beams::state::State;

pub(crate) fn ramp_response(ei: f64, k: f64, x: f64) -> State {
    let [_, _, c3, c4, c5] = coefficients(k * x);
    let x2 = x * x;
    State::new(
        x2 * x2 * x * c5 / ei,
        x2 * x2 * c4 / ei,
        x2 * x * c3,
        x2 / 2.0,
    )
}
