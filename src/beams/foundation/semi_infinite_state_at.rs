use super::SemiInfiniteSolution;
use super::load_response::load_response;
use super::unit_responses::unit_responses;
use crate::beams::state::State;

impl SemiInfiniteSolution {
    /// State `[y, θ, M, V]` at `x ≥ 0`.
    pub fn state_at(&self, x: f64) -> State {
        let beam = &self.beam;
        let responses = unit_responses(beam.ei, beam.modulus, x, false);
        let conditioning: State =
            responses.column(3) * self.end_force + responses.column(2) * self.end_couple;
        beam.loads.iter().fold(conditioning, |acc, load| {
            acc + load_response(beam.ei, beam.modulus, load, x, false)
        })
    }
}
