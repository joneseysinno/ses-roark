use super::Span;
use crate::beams::beam::{Beam, BeamSolution};
use crate::beams::end_condition::EndCondition;
use crate::beams::error::BeamError;

/// The span solved as simply supported under its own loads.
pub(crate) fn simple_span(span: &Span) -> Result<BeamSolution, BeamError> {
    Beam::new(
        span.length,
        span.ei,
        EndCondition::Pinned,
        EndCondition::Pinned,
        span.loads.clone(),
    )
    .solve()
}
