use super::RestrainedSolution;
use crate::beams::beam::{Beam, BeamSolution};
use crate::beams::error::BeamError;
use crate::beams::evaluation_points::evaluation_points;
use crate::beams::kernel::Kernel;
use crate::beams::kernel::variable_section::integrate::integrate;
use crate::beams::segment_count::MAX_CHARACTERISTIC_EXTENT;

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
    let solve_with = |tension: f64| {
        let solved = Beam {
            kernel: if tension > 0.0 {
                Kernel::AxialTension { force: tension }
            } else {
                Kernel::Plain
            },
            ..beam.clone()
        }
        .solve();
        match solved {
            Ok(solution) => Ok(solution),
            Err(BeamError::InvalidInput(message)) => Err(BeamError::InvalidInput(message)),
            Err(BeamError::UnsupportedCombination(message)) => {
                Err(BeamError::UnsupportedCombination(message))
            }
            Err(_) => Err(BeamError::NoConvergence("restrained-end tension")),
        }
    };
    let induced = |solution: &BeamSolution| {
        let points = evaluation_points(solution);
        axial_rigidity / (2.0 * length)
            * integrate(|x| solution.slope(x).powi(2), 0.0, length, &points)
    };
    // A trial past the shooting cap is stiffer than any beam we can integrate,
    // so the induced tension lies below it and the bracket moves down.
    let too_stiff = |tension: f64| {
        beam.stiffness
            .uniform_value()
            .is_some_and(|ei| (tension / ei).sqrt() * length > MAX_CHARACTERISTIC_EXTENT)
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
        if too_stiff(mid) {
            hi = mid;
        } else if mid < induced(&solve_with(mid)?) {
            lo = mid;
        } else {
            hi = mid;
        }
        if hi - lo <= 1e-14 * upper {
            let tension = 0.5 * (lo + hi);
            let solution = solve_with(tension)?;
            let consistent = (tension - induced(&solution)).abs() <= 1e-8 * upper;
            if !consistent {
                return Err(BeamError::NoConvergence("restrained-end tension"));
            }
            return Ok(RestrainedSolution { tension, solution });
        }
    }
    Err(BeamError::NoConvergence("restrained-end tension"))
}
