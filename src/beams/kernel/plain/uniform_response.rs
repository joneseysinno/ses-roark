use crate::beams::state::State;

pub(crate) fn uniform_response(ei: f64, shear_flexibility: f64, x: f64) -> State {
    let x2 = x * x;
    State::new(
        x2 * x2 / (24.0 * ei) - shear_flexibility * x2 / 2.0,
        x2 * x / (6.0 * ei),
        x2 / 2.0,
        x,
    )
}
