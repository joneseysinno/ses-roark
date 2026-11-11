use crate::beams::beam::Beam;
use crate::beams::error::BeamError;
use crate::beams::ratios_between::ratios_between;
use crate::beams::response_ratios::ResponseRatios;
use crate::beams::stiffness::Stiffness;

/// Table 8.11 coefficients: responses of the variable-section beam divided by
/// those of the same beam with uniform `EI` equal to its value at the left end.
pub fn taper_coefficients(beam: &Beam) -> Result<ResponseRatios, BeamError> {
    let reference_ei = beam.stiffness.ei_at(0.0, beam.length);
    if !(reference_ei > 0.0 && reference_ei.is_finite()) {
        return Err(BeamError::InvalidInput(format!(
            "reference EI at the left end must be positive, got {reference_ei}"
        )));
    }
    let subject = beam.solve()?;
    let reference = Beam {
        stiffness: Stiffness::Uniform(reference_ei),
        ..beam.clone()
    }
    .solve()?;
    Ok(ratios_between(&subject, &reference))
}
