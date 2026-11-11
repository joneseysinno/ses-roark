use super::beam::BeamSolution;
use super::local_loads::segment_load_terms;
use super::state::State;

impl BeamSolution {
    /// Full state `[y, θ, M, V]` at station `x`. At a point action the value
    /// includes that action (right-hand limit).
    pub fn state_at(&self, x: f64) -> State {
        if self.nodes.is_empty() {
            return self
                .beam
                .loads
                .iter()
                .fold(self.kernel.transfer(x) * self.initial, |acc, load| {
                    acc + self.kernel.load_terms(load, x)
                });
        }
        let n = self.nodes.len();
        let i = if !x.is_finite() || x <= 0.0 {
            0
        } else {
            ((x / self.step).floor() as usize).min(n - 1)
        };
        let x0 = i as f64 * self.step;
        let last = i + 1 == n;
        let x1 = if last {
            self.beam.length
        } else {
            x0 + self.step
        };
        self.kernel.transfer(x - x0) * self.nodes[i]
            + segment_load_terms(&self.kernel, &self.beam.loads, x0, x1, x, last)
    }
}
