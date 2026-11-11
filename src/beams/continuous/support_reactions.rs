use super::ContinuousSolution;
use crate::beams::state::SHEAR;

impl ContinuousSolution {
    /// Upward vertical reaction at each support.
    pub fn support_reactions(&self) -> Vec<f64> {
        let mut reactions = vec![0.0; self.spans.len() + 1];
        for (i, span) in self.spans.iter().enumerate() {
            reactions[i] += span.initial[SHEAR];
            reactions[i + 1] -= span.state_at(span.beam.length)[SHEAR];
        }
        reactions
    }
}
