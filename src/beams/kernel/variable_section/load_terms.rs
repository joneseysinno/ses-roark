use super::super::Resolved;
use super::integrate::integrate;
use crate::beams::load::Load;
use crate::beams::state::{MOMENT, SHEAR, State};
use crate::beams::stiffness::Stiffness;

/// Moment and shear load terms do not depend on `EI`; slope and deflection
/// integrate the load moment over `EI(ξ)`. Kinematic actions are `EI`-free.
pub(crate) fn load_terms(
    stiffness: &Stiffness,
    length: f64,
    shear_flexibility: f64,
    breakpoints: &[f64],
    load: &Load,
    x: f64,
) -> State {
    let unit = Resolved::Plain {
        ei: 1.0,
        shear_flexibility: 0.0,
    };
    let direct = unit.load_terms(load, x);
    if !matches!(
        load,
        Load::PointForce { .. } | Load::LinearLoad { .. } | Load::PointMoment { .. }
    ) {
        return direct;
    }
    let moment = |xi: f64| unit.load_terms(load, xi)[MOMENT];
    let flexibility = |xi: f64| 1.0 / stiffness.ei_at(xi, length);
    let slope = integrate(|xi| moment(xi) * flexibility(xi), 0.0, x, breakpoints);
    let bending = integrate(
        |xi| (x - xi) * moment(xi) * flexibility(xi),
        0.0,
        x,
        breakpoints,
    );
    let sliding = if matches!(load, Load::PointMoment { .. }) {
        0.0
    } else {
        -shear_flexibility * direct[MOMENT]
    };
    State::new(bending + sliding, slope, direct[MOMENT], direct[SHEAR])
}
