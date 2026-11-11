use std::f64::consts::PI;

use crate::beams::assemble_system::assemble_system;
use crate::beams::beam::Beam;
use crate::beams::error::BeamError;
use crate::beams::kernel::{Kernel, Resolved};

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
    let determinant = |force: f64| {
        let probe = Beam {
            kernel: Kernel::AxialCompression { force },
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
    const STEPS: usize = 2500;
    let start = 1e-6 * euler;
    let mut previous_force = start;
    let mut previous = determinant(start);
    if previous.abs() < 1e-12 {
        return Err(BeamError::UnstableSupports);
    }
    for i in 1..=STEPS {
        let force = 25.0 * euler * i as f64 / STEPS as f64;
        let current = determinant(force);
        if current.signum() != previous.signum() {
            let (mut lo, mut hi) = (previous_force, force);
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
            return Ok(0.5 * (lo + hi));
        }
        previous_force = force;
        previous = current;
    }
    Ok(f64::INFINITY)
}
