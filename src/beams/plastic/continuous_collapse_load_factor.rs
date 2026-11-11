use super::collapse_load_factor::collapse_load_factor;
use crate::beams::continuous::ContinuousBeam;
use crate::beams::end_condition::EndCondition;
use crate::beams::error::BeamError;

/// Collapse load factor of a continuous beam with uniform plastic moment:
/// the least span beam mechanism, with hinges over interior supports.
pub fn continuous_collapse_load_factor(
    beam: &ContinuousBeam,
    plastic_moment: f64,
) -> Result<f64, BeamError> {
    let n = beam.spans.len();
    let mut best = f64::INFINITY;
    for (i, span) in beam.spans.iter().enumerate() {
        if span.loads.is_empty() {
            continue;
        }
        let left = if i == 0 {
            beam.left
        } else {
            EndCondition::Fixed
        };
        let right = if i + 1 == n {
            beam.right
        } else {
            EndCondition::Fixed
        };
        best = best.min(collapse_load_factor(
            span.length,
            left,
            right,
            &span.loads,
            plastic_moment,
        )?);
    }
    if best.is_finite() {
        Ok(best)
    } else {
        Err(BeamError::InvalidInput("no span carries load".into()))
    }
}
