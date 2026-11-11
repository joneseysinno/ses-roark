use super::Resolved;
use crate::beams::load::Load;
use crate::beams::state::State;

/// Load terms for constant-coefficient kernels.
///
/// Every action is a jump in the state: a downward force `W` is `−W` in `V`,
/// a counterclockwise couple `M_o` is `−M_o` in `M`, a kink is `+θ_o` in `θ`,
/// an offset is `+Δ_o` in `y`. Point actions propagate as `T(x − a)·jump`;
/// distributed actions integrate that, using the kernel's closed-form
/// uniform / ramp / curvature responses.
pub(crate) fn shift_invariant_load_terms(kernel: &Resolved, load: &Load, x: f64) -> State {
    let point = |at: f64, jump: State| {
        if x >= at {
            kernel.transfer(x - at) * jump
        } else {
            State::zeros()
        }
    };
    let from = |start: f64, response: &dyn Fn(f64) -> State| {
        if x >= start {
            response(x - start)
        } else {
            State::zeros()
        }
    };
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
            let uniform = |v: f64| kernel.uniform_response(v);
            let ramp = |v: f64| kernel.ramp_response(v);
            -(from(start, &uniform) * w_start + from(start, &ramp) * slope
                - from(end, &uniform) * w_end
                - from(end, &ramp) * slope)
        }
        Load::ImposedCurvature {
            curvature,
            start,
            end,
        } => {
            if end <= start {
                return State::zeros();
            }
            let response = |v: f64| kernel.curvature_response(v);
            (from(start, &response) - from(end, &response)) * curvature
        }
    }
}
