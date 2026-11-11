use super::kernel::Resolved;
use super::load::Load;
use super::state::State;

/// Load terms at global station `x` from the actions that belong to `[x0, x1)`.
///
/// The last segment also takes an action sitting at `x1`. Point actions are
/// admitted with the global test `x >= at` and then carried by
/// `T(max(x − at, 0))`, so a query exactly on the action is the right-hand
/// limit whichever segment contains it. Distributed actions are clipped to the
/// segment, shifted to a local origin at `x0`, and integrated from there.
pub(crate) fn segment_load_terms(
    kernel: &Resolved,
    loads: &[Load],
    x0: f64,
    x1: f64,
    x: f64,
    last: bool,
) -> State {
    let inside = |at: f64| at >= x0 && (at < x1 || (last && at <= x1));
    let mut acc = State::zeros();
    for load in loads {
        match *load {
            Load::PointForce { magnitude, at } if inside(at) && x >= at => {
                acc +=
                    kernel.load_terms(&Load::PointForce { magnitude, at: 0.0 }, (x - at).max(0.0));
            }
            Load::PointMoment { magnitude, at } if inside(at) && x >= at => {
                acc +=
                    kernel.load_terms(&Load::PointMoment { magnitude, at: 0.0 }, (x - at).max(0.0));
            }
            Load::AngularDeformation { angle, at } if inside(at) && x >= at => {
                acc += kernel.load_terms(
                    &Load::AngularDeformation { angle, at: 0.0 },
                    (x - at).max(0.0),
                );
            }
            Load::LateralDisplacement { offset, at } if inside(at) && x >= at => {
                acc += kernel.load_terms(
                    &Load::LateralDisplacement { offset, at: 0.0 },
                    (x - at).max(0.0),
                );
            }
            Load::LinearLoad {
                w_start,
                w_end,
                start,
                end,
            } if end > start => {
                let (a, b) = (start.max(x0), end.min(x1));
                if b > a {
                    let intensity =
                        |t: f64| w_start + (w_end - w_start) * (t - start) / (end - start);
                    acc += kernel.load_terms(
                        &Load::LinearLoad {
                            w_start: intensity(a),
                            w_end: intensity(b),
                            start: a - x0,
                            end: b - x0,
                        },
                        x - x0,
                    );
                }
            }
            Load::ImposedCurvature {
                curvature,
                start,
                end,
            } => {
                let (a, b) = (start.max(x0), end.min(x1));
                if b > a {
                    acc += kernel.load_terms(
                        &Load::ImposedCurvature {
                            curvature,
                            start: a - x0,
                            end: b - x0,
                        },
                        x - x0,
                    );
                }
            }
            _ => {}
        }
    }
    acc
}
