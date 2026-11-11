use roark::beams::state::{DEFLECTION, MOMENT, SHEAR, SLOPE};
use roark::beams::{EndCondition, State};

/// The two quantities an end condition forces to zero, evaluated on `state`.
pub fn end_residuals(end: EndCondition, state: &State) -> [f64; 2] {
    match end {
        EndCondition::Free => [state[MOMENT], state[SHEAR]],
        EndCondition::Guided => [state[SLOPE], state[SHEAR]],
        EndCondition::Pinned => [state[DEFLECTION], state[MOMENT]],
        EndCondition::Fixed => [state[DEFLECTION], state[SLOPE]],
        EndCondition::Elastic { .. } => panic!("elastic ends have no zero residuals"),
    }
}
