use super::coefficients::coefficients;
use crate::beams::state::State;

pub(crate) fn uniform_response(ei: f64, k: f64, x: f64) -> State {
    let [_, h2, h3, h4, _] = coefficients(k * x);
    let x2 = x * x;
    State::new(x2 * x2 * h4 / ei, x2 * x * h3 / ei, x2 * h2, x)
}
