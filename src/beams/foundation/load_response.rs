use super::system_inverse::system_inverse;
use super::unit_responses::unit_responses;
use crate::beams::load::Load;
use crate::beams::state::State;

/// Infinite-beam state at `x` due to one action.
///
/// Distributed actions use exact antiderivatives of the unit responses:
/// `Q(t) = A⁻¹(R(t)·j − j·H(t))` integrates `R` from `−∞`, and
/// `P(t) = A⁻¹·Q(t) − A⁻¹·j·t·H(t)` integrates `Q`.
pub(crate) fn load_response(ei: f64, modulus: f64, load: &Load, x: f64, left_limit: bool) -> State {
    let inverse = system_inverse(ei, modulus);
    let step = |t: f64| if t >= 0.0 { 1.0 } else { 0.0 };
    let q = |t: f64, jump: State| {
        inverse * (unit_responses(ei, modulus, t, false) * jump - jump * step(t))
    };
    let p = |t: f64, jump: State| inverse * q(t, jump) - inverse * jump * (t * step(t));
    let point = |at: f64, jump: State| unit_responses(ei, modulus, x - at, left_limit) * jump;
    match *load {
        Load::PointForce { magnitude, at } => point(at, State::new(0.0, 0.0, 0.0, -magnitude)),
        Load::PointMoment { magnitude, at } => point(at, State::new(0.0, 0.0, -magnitude, 0.0)),
        Load::AngularDeformation { angle, at } => point(at, State::new(0.0, angle, 0.0, 0.0)),
        Load::LateralDisplacement { offset, at } => point(at, State::new(offset, 0.0, 0.0, 0.0)),
        Load::LinearLoad {
            w_start,
            w_end,
            start,
            end,
        } => {
            if end <= start {
                return State::zeros();
            }
            let slope = (w_end - w_start) / (end - start);
            let shear = State::new(0.0, 0.0, 0.0, 1.0);
            let (t1, t2) = (x - end, x - start);
            let level = (q(t2, shear) - q(t1, shear)) * (w_start + slope * (x - start));
            let first_moment =
                (q(t2, shear) * t2 - p(t2, shear)) - (q(t1, shear) * t1 - p(t1, shear));
            -(level - first_moment * slope)
        }
        Load::ImposedCurvature {
            curvature,
            start,
            end,
        } => {
            if end <= start {
                return State::zeros();
            }
            let kink = State::new(0.0, 1.0, 0.0, 0.0);
            (q(x - start, kink) - q(x - end, kink)) * curvature
        }
    }
}
