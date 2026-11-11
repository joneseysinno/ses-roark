use super::integrate::integrate;
use crate::beams::state::StateMatrix;
use crate::beams::stiffness::Stiffness;

pub(crate) fn transfer(
    stiffness: &Stiffness,
    length: f64,
    shear_flexibility: f64,
    breakpoints: &[f64],
    x: f64,
) -> StateMatrix {
    let flexibility = |xi: f64| 1.0 / stiffness.ei_at(xi, length);
    let i0 = integrate(flexibility, 0.0, x, breakpoints);
    let i1 = integrate(|xi| xi * flexibility(xi), 0.0, x, breakpoints);
    let i2 = integrate(|xi| xi * xi * flexibility(xi), 0.0, x, breakpoints);
    StateMatrix::new(
        1.0,
        x,
        x * i0 - i1,
        x * i1 - i2 - shear_flexibility * x,
        0.0,
        1.0,
        i0,
        i1,
        0.0,
        0.0,
        1.0,
        x,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
