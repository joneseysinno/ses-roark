use super::beam::BeamSolution;
use super::state::State;

impl BeamSolution {
    /// Full state `[y, θ, M, V]` at station `x`. At a point action the value
    /// includes that action (right-hand limit).
    pub fn state_at(&self, x: f64) -> State {
        self.beam
            .loads
            .iter()
            .fold(self.kernel.transfer(x) * self.initial, |acc, load| {
                acc + self.kernel.load_terms(load, x)
            })
    }
}
