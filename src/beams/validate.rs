use super::beam::Beam;
use super::end_condition::EndCondition;
use super::error::BeamError;
use super::kernel::Kernel;
use super::load::Load;
use super::stiffness::Stiffness;

/// Reject non-physical geometry, rigidities, spring constants, and loads.
pub(crate) fn validate(beam: &Beam) -> Result<(), BeamError> {
    let positive = |v: f64| v > 0.0 && v.is_finite();
    let non_negative = |v: f64| v >= 0.0;
    let fail = |msg: String| Err(BeamError::InvalidInput(msg));
    let l = beam.length;
    if !positive(l) {
        return fail(format!("length must be positive and finite, got {l}"));
    }
    let stiffness_ok = match &beam.stiffness {
        Stiffness::Uniform(ei) => positive(*ei),
        Stiffness::Piecewise(segments) => {
            !segments.is_empty()
                && segments.iter().all(|(_, ei)| positive(*ei))
                && segments.windows(2).all(|pair| pair[1].0 > pair[0].0)
        }
        Stiffness::Tapered {
            ei_start,
            ei_end,
            exponent,
        } => {
            non_negative(*ei_start)
                && non_negative(*ei_end)
                && positive(ei_start.max(*ei_end))
                && positive(*exponent)
        }
    };
    if !stiffness_ok {
        return fail(format!("invalid stiffness {:?}", beam.stiffness));
    }
    if let Some(rigidity) = beam.shear_rigidity
        && !positive(rigidity)
    {
        return fail(format!("shear rigidity must be positive, got {rigidity}"));
    }
    let kernel_ok = match beam.kernel {
        Kernel::Plain => true,
        Kernel::ElasticFoundation { modulus } => non_negative(modulus) && modulus.is_finite(),
        Kernel::AxialCompression { force } | Kernel::AxialTension { force } => force.is_finite(),
    };
    if !kernel_ok {
        return fail(format!("invalid kernel parameters {:?}", beam.kernel));
    }
    for end in [beam.left, beam.right] {
        if let EndCondition::Elastic {
            vertical,
            rotational,
        } = end
            && !(non_negative(vertical) && non_negative(rotational))
        {
            return fail(format!(
                "spring constants must be non-negative, got {end:?}"
            ));
        }
    }
    let tol = 1e-12 * l;
    let inside = |x: f64| x >= -tol && x <= l + tol;
    for load in &beam.loads {
        let ok = match *load {
            Load::PointForce { at, .. }
            | Load::PointMoment { at, .. }
            | Load::AngularDeformation { at, .. }
            | Load::LateralDisplacement { at, .. } => inside(at),
            Load::LinearLoad { start, end, .. } | Load::ImposedCurvature { start, end, .. } => {
                inside(start) && inside(end) && start <= end
            }
        };
        if !ok {
            return fail(format!("load {load:?} does not lie on the span 0..{l}"));
        }
    }
    Ok(())
}
