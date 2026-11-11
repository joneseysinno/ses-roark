use crate::beams::state::State;

pub(crate) fn curvature_response(x: f64) -> State {
    State::new(x * x / 2.0, x, 0.0, 0.0)
}
