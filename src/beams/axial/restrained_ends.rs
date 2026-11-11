use super::RestrainedSolution;
use crate::beams::beam::{Beam, BeamSolution};
use crate::beams::breakpoints::breakpoints;
use crate::beams::error::BeamError;
use crate::beams::kernel::Kernel;
use crate::beams::kernel::variable_section::integrate::integrate;

/// Ends restrained against horizontal displacement (Table 8.9): the
/// deflection stretches the axis, inducing tension
/// `P = EA/(2l)·∫θ² dx`, which in turn stiffens the beam. `P` is found by
/// bisection on `P − EA/(2l)·∫θ²(P)`, which is increasing in `P`.
pub fn restrained_ends(beam: &Beam, axial_rigidity: f64) -> Result<RestrainedSolution, BeamError> {
    if beam.kernel != Kernel::Plain {
        return Err(BeamError::UnsupportedCombination(
            "restrained-end analysis starts from a plain-kernel beam",
        ));
    }
    if !(axial_rigidity > 0.0 && axial_rigidity.is_finite()) {
        return Err(BeamError::InvalidInput(format!(
            "axial rigidity must be positive, got {axial_rigidity}"
        )));
    }
    let length = beam.length;
    let points = breakpoints(beam);
    let solve_with = |tension: f64| {
        Beam {
            kernel: if tension > 0.0 {
                Kernel::AxialTension { force: tension }
            } else {
                Kernel::Plain
            },
            ..beam.clone()
        }
        .solve()
    };
    let induced = |solution: &BeamSolution| {
        axial_rigidity / (2.0 * length)
            * integrate(|x| solution.slope(x).powi(2), 0.0, length, &points)
    };
    let unstretched = solve_with(0.0)?;
    let upper = induced(&unstretched);
    if upper == 0.0 {
        return Ok(RestrainedSolution {
            tension: 0.0,
            solution: unstretched,
        });
    }
    let (mut lo, mut hi) = (0.0, upper);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if mid < induced(&solve_with(mid)?) {
            lo = mid;
        } else {
            hi = mid;
        }
        if hi - lo <= 1e-14 * upper {
            let tension = 0.5 * (lo + hi);
            return Ok(RestrainedSolution {
                tension,
                solution: solve_with(tension)?,
            });
        }
    }
    Err(BeamError::NoConvergence("restrained-end tension"))
}
