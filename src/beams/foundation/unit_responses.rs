use super::characteristic::characteristic;
use super::system_matrix::system_matrix;
use crate::beams::state::{State, StateMatrix};

/// Decaying infinite-beam responses at offset `t` from a unit jump in each
/// state component: column `j` is the response to a jump `e_j` at `t = 0`.
///
/// The `V` column is minus the classic point-load solution
/// `y = −(Pβ/2k)·e^(−βt)(cos βt + sin βt)`; the others follow from
/// `R_{A·j} = A·R_j`, since the derivative of a decaying response is itself
/// a decaying response. At `t = 0` the right-hand limit is used unless
/// `left_limit` is set.
pub(crate) fn unit_responses(ei: f64, modulus: f64, t: f64, left_limit: bool) -> StateMatrix {
    let beta = characteristic(ei, modulus);
    let u = beta * t.abs();
    let decay = (-u).exp();
    let (s, c) = u.sin_cos();
    let (a, b, cc, d) = (decay * (c + s), decay * s, decay * (c - s), decay * c);
    let side = if t > 0.0 || (t == 0.0 && !left_limit) {
        1.0
    } else {
        -1.0
    };
    let downward_unit_force = State::new(
        -beta / (2.0 * modulus) * a,
        side * beta * beta / modulus * b,
        cc / (4.0 * beta),
        -side * d / 2.0,
    );
    let system = system_matrix(ei, modulus);
    let shear_jump = -downward_unit_force;
    let moment_jump = system * shear_jump;
    let slope_jump = system * moment_jump * ei;
    let deflection_jump = system * slope_jump;
    StateMatrix::from_columns(&[deflection_jump, slope_jump, moment_jump, shear_jump])
}
