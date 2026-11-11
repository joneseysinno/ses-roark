use super::coefficients::coefficients;
use crate::beams::state::State;

pub(crate) fn uniform_response(ei: f64, k: f64, x: f64) -> State {
    let [_, c2, c3, c4, _] = coefficients(k * x);
    let x2 = x * x;
    State::new(x2 * x2 * c4 / ei, x2 * x * c3 / ei, x2 * c2, x)
}
