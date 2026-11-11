use nalgebra::SVector;

use super::Member;
use crate::beams::beam::Beam;
use crate::beams::end_condition::EndCondition;
use crate::beams::error::BeamError;
use crate::beams::state::{MOMENT, SHEAR};

/// Local end actions exerted on a fully fixed member by its supports under
/// the member loads: `[0, R_A, −M_A, 0, R_B, M_B]` from a fixed-fixed solve
/// (internal end moments converted to counterclockwise end couples).
pub(crate) fn fixed_end_actions(
    member: &Member,
    length: f64,
) -> Result<SVector<f64, 6>, BeamError> {
    if member.loads.is_empty() {
        return Ok(SVector::zeros());
    }
    let solution = Beam::new(
        length,
        member.modulus * member.inertia,
        EndCondition::Fixed,
        EndCondition::Fixed,
        member.loads.clone(),
    )
    .solve()?;
    let start = solution.initial;
    let end = solution.state_at(length);
    Ok(SVector::<f64, 6>::from_column_slice(&[
        0.0,
        start[SHEAR],
        -start[MOMENT],
        0.0,
        -end[SHEAR],
        end[MOMENT],
    ]))
}
