use super::beam::{BeamSolution, Reactions};
use super::state::{MOMENT, SHEAR};

impl BeamSolution {
    /// Support reactions: `R_A = V_A`, `M_A = M(0)`, `R_B = −V(l)`, `M_B = M(l)`.
    pub fn reactions(&self) -> Reactions {
        let end = self.state_at(self.beam.length);
        let (left, right) = (self.beam.left, self.beam.right);
        Reactions {
            ra: left
                .provides_vertical_reaction()
                .then_some(self.initial[SHEAR]),
            ma: left
                .provides_moment_reaction()
                .then_some(self.initial[MOMENT]),
            rb: right.provides_vertical_reaction().then_some(-end[SHEAR]),
            mb: right.provides_moment_reaction().then_some(end[MOMENT]),
        }
    }
}
