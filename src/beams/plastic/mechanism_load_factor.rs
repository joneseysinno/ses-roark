use super::external_work::external_work;
use crate::beams::load::Load;

/// Load factor of the beam mechanism with an interior hinge at `hinge` and
/// optional hinges at the ends: internal work `M_p·Σθ` over external work.
pub(crate) fn mechanism_load_factor(
    length: f64,
    hinged_ends: (bool, bool),
    loads: &[Load],
    plastic_moment: f64,
    hinge: f64,
) -> f64 {
    let left = 1.0 / hinge;
    let right = 1.0 / (length - hinge);
    let mut rotation = left + right;
    if hinged_ends.0 {
        rotation += left;
    }
    if hinged_ends.1 {
        rotation += right;
    }
    let work = external_work(loads, length, hinge);
    if work > 0.0 {
        plastic_moment * rotation / work
    } else {
        f64::INFINITY
    }
}
