use super::InfiniteBeam;
use super::load_response::load_response;
use crate::beams::state::State;

impl InfiniteBeam {
    /// State `[y, θ, M, V]` at `x`.
    pub fn state_at(&self, x: f64) -> State {
        self.loads.iter().fold(State::zeros(), |acc, load| {
            acc + load_response(self.ei, self.modulus, load, x, false)
        })
    }
}
