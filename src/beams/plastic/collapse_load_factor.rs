use super::mechanism_load_factor::mechanism_load_factor;
use crate::beams::end_condition::EndCondition;
use crate::beams::error::BeamError;
use crate::beams::load::Load;
use crate::beams::sum_moments::sum_moments;

/// Factor on `loads` at which a single span of uniform plastic moment
/// `plastic_moment` collapses. Ends are `Free`, `Pinned`, or `Fixed`; loads
/// are downward point or distributed forces.
pub fn collapse_load_factor(
    length: f64,
    left: EndCondition,
    right: EndCondition,
    loads: &[Load],
    plastic_moment: f64,
) -> Result<f64, BeamError> {
    if loads
        .iter()
        .any(|load| !matches!(load, Load::PointForce { .. } | Load::LinearLoad { .. }))
    {
        return Err(BeamError::UnsupportedCombination(
            "collapse analysis supports point and distributed forces only",
        ));
    }
    use EndCondition::{Fixed, Free, Pinned};
    let factor = match (left, right) {
        (Fixed, Free) => plastic_moment / -sum_moments(loads, 0.0),
        (Free, Fixed) => plastic_moment / sum_moments(loads, length),
        (Pinned | Fixed, Pinned | Fixed) => {
            let hinged = (left == Fixed, right == Fixed);
            let lambda = |x: f64| mechanism_load_factor(length, hinged, loads, plastic_moment, x);
            let mut points = vec![0.0, length];
            for load in loads {
                match *load {
                    Load::PointForce { at, .. } => points.push(at),
                    Load::LinearLoad { start, end, .. } => points.extend([start, end]),
                    _ => {}
                }
            }
            points.retain(|x| (0.0..=length).contains(x));
            points.sort_by(f64::total_cmp);
            points.dedup();
            let golden = 0.5 * (5.0_f64.sqrt() - 1.0);
            let mut best = f64::INFINITY;
            for pair in points.windows(2) {
                let (mut lo, mut hi) = (pair[0], pair[1]);
                if pair[1] < length {
                    best = best.min(lambda(pair[1]));
                }
                let mut c = hi - golden * (hi - lo);
                let mut d = lo + golden * (hi - lo);
                let (mut fc, mut fd) = (lambda(c), lambda(d));
                for _ in 0..120 {
                    if fc < fd {
                        hi = d;
                        d = c;
                        fd = fc;
                        c = hi - golden * (hi - lo);
                        fc = lambda(c);
                    } else {
                        lo = c;
                        c = d;
                        fc = fd;
                        d = lo + golden * (hi - lo);
                        fd = lambda(d);
                    }
                }
                best = best.min(lambda(0.5 * (lo + hi)));
            }
            best
        }
        _ => {
            return Err(BeamError::UnsupportedCombination(
                "collapse analysis needs a cantilever or a span supported at both ends",
            ));
        }
    };
    if factor.is_finite() && factor > 0.0 {
        Ok(factor)
    } else {
        Err(BeamError::InvalidInput(
            "loads do no positive work on any mechanism".into(),
        ))
    }
}
