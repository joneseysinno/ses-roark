use super::coefficients::coefficients;
use crate::beams::state::State;

pub(crate) fn ramp_response(ei: f64, k: f64, x: f64) -> State {
    let [_, _, h3, h4, h5] = coefficients(k * x);
    let x2 = x * x;
    State::new(
        x2 * x2 * x * h5 / ei,
        x2 * x2 * h4 / ei,
        x2 * x * h3,
        x2 / 2.0,
    )
}
