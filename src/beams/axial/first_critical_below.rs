use std::f64::consts::PI;

use crate::beams::assemble_system::assemble_system;
use crate::beams::beam::Beam;
use crate::beams::error::BeamError;
use crate::beams::kernel::{Kernel, Resolved};

/// First critical load in `(0, force]`, or `None` when the boundary
/// determinant does not change sign on that interval.
///
/// The probe grid and the bisection are the same ones [`super::critical_load`]
/// uses over its full scan.
pub fn first_critical_below(beam: &Beam, force: f64) -> Result<Option<f64>, BeamError> {
    let Some(ei) = beam.stiffness.uniform_value() else {
        return Err(BeamError::UnsupportedCombination(
            "critical load requires uniform stiffness",
        ));
    };
    if force <= 0.0 || !force.is_finite() {
        return Ok(None);
    }
    let euler = PI * PI * ei / (beam.length * beam.length);
    let determinant = |compression: f64| {
        let probe = Beam {
            kernel: Kernel::AxialCompression { force: compression },
            shear_rigidity: None,
            loads: Vec::new(),
            ..beam.clone()
        };
        let Ok(kernel) = Resolved::from_beam(&probe) else {
            return f64::NAN;
        };
        let (mut matrix, _) = assemble_system(&kernel, &probe);
        for i in 0..4 {
            let scale = matrix.row(i).amax();
            if scale > 0.0 {
                matrix.row_mut(i).unscale_mut(scale);
            }
        }
        matrix.determinant()
    };
    let start = 1e-6 * euler;
    if force < start {
        return Ok(None);
    }
    let mut previous_force = start;
    let mut previous = determinant(start);
    if previous.abs() < 1e-12 {
        return Err(BeamError::UnstableSupports);
    }
    let mut i = 1usize;
    loop {
        let grid = 25.0 * euler * i as f64 / 2500.0;
        let at = if grid < force { grid } else { force };
        let current = determinant(at);
        if current.signum() != previous.signum() {
            let (mut lo, mut hi) = (previous_force, at);
            for _ in 0..200 {
                let mid = 0.5 * (lo + hi);
                if determinant(mid).signum() == previous.signum() {
                    lo = mid;
                } else {
                    hi = mid;
                }
                if hi - lo <= 1e-15 * hi {
                    break;
                }
            }
            return Ok(Some(0.5 * (lo + hi)));
        }
        if grid >= force {
            return Ok(None);
        }
        previous_force = at;
        previous = current;
        i += 1;
    }
}
