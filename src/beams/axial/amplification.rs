use crate::beams::beam::Beam;
use crate::beams::error::BeamError;
use crate::beams::kernel::Kernel;
use crate::beams::ratios_between::ratios_between;
use crate::beams::response_ratios::ResponseRatios;

/// Table 8.10 coefficients: responses of the axially loaded beam divided by
/// those of the same beam without axial load.
pub fn amplification(beam: &Beam) -> Result<ResponseRatios, BeamError> {
    let subject = beam.solve()?;
    let reference = Beam {
        kernel: Kernel::Plain,
        ..beam.clone()
    }
    .solve()?;
    Ok(ratios_between(&subject, &reference))
}
