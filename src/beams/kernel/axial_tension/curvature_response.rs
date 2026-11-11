use super::coefficients::coefficients;
use crate::beams::state::State;

pub(crate) fn curvature_response(force: f64, k: f64, x: f64) -> State {
    let [h1, h2, _, _, _] = coefficients(k * x);
    let x2 = x * x;
    State::new(x2 * h2, x * h1, force * x2 * h2, 0.0)
}
