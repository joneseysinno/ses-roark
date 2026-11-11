use nalgebra::SVector;

use super::Member;
use crate::beams::beam::{Beam, BeamSolution};
use crate::beams::end_condition::EndCondition;
use crate::beams::kernel::Resolved;
use crate::beams::state::State;

/// Transverse beam solution of a member from its local end displacements and
/// end forces: `s_A = [v1, θ1, −M1, V1]`.
pub(crate) fn member_solution(
    member: &Member,
    length: f64,
    displacements: &SVector<f64, 6>,
    forces: &SVector<f64, 6>,
) -> BeamSolution {
    let ei = member.modulus * member.inertia;
    let beam = Beam::new(
        length,
        ei,
        EndCondition::Fixed,
        EndCondition::Fixed,
        member.loads.clone(),
    );
    let kernel = Resolved::Plain {
        ei,
        shear_flexibility: 0.0,
    };
    let initial = State::new(displacements[1], displacements[2], -forces[2], forces[1]);
    BeamSolution::from_initial_state(beam, kernel, initial)
}
