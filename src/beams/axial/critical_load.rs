use std::f64::consts::PI;

use super::first_critical_below::first_critical_below;
use crate::beams::beam::Beam;
use crate::beams::error::BeamError;

/// Lowest compressive end load at which the beam's end conditions admit a
/// non-trivial deflected shape: the first sign change of the normalized
/// boundary determinant, refined by bisection. Returns `f64::INFINITY` when
/// no critical load lies below `25·π²EI/l²`.
pub fn critical_load(beam: &Beam) -> Result<f64, BeamError> {
    let Some(ei) = beam.stiffness.uniform_value() else {
        return Err(BeamError::UnsupportedCombination(
            "critical load requires uniform stiffness",
        ));
    };
    let euler = PI * PI * ei / (beam.length * beam.length);
    Ok(first_critical_below(beam, 25.0 * euler)?.unwrap_or(f64::INFINITY))
}
