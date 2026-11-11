use super::beam::Beam;
use super::end_condition::Side;
use super::end_equations::end_rows;
use super::kernel::Resolved;
use super::state::{State, StateMatrix};

/// 4x4 system for the left-end state. Left rows act on the state just before
/// any action applied at `x = 0`; right rows act on `s(l)` including actions
/// applied at `x = l`.
pub(crate) fn assemble_system(kernel: &Resolved, beam: &Beam) -> (StateMatrix, State) {
    let l = beam.length;
    let transfer = kernel.transfer(l);
    let loads_at_end = beam
        .loads
        .iter()
        .fold(State::zeros(), |acc, load| acc + kernel.load_terms(load, l));
    let mut matrix = StateMatrix::zeros();
    let mut rhs = State::zeros();
    for (i, row) in end_rows(beam.left, Side::Left).iter().enumerate() {
        matrix.set_row(i, row);
    }
    for (i, row) in end_rows(beam.right, Side::Right).iter().enumerate() {
        matrix.set_row(2 + i, &(row * transfer));
        rhs[2 + i] = -(row * loads_at_end)[(0, 0)];
    }
    (matrix, rhs)
}
