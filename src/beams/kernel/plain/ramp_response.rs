use crate::beams::state::State;

pub(crate) fn ramp_response(ei: f64, shear_flexibility: f64, x: f64) -> State {
    let x2 = x * x;
    let x3 = x2 * x;
    State::new(
        x3 * x2 / (120.0 * ei) - shear_flexibility * x3 / 6.0,
        x2 * x2 / (24.0 * ei),
        x3 / 6.0,
        x2 / 2.0,
    )
}
