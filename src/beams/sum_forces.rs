use super::load::Load;

/// Net applied vertical force, positive downward (kinematic actions contribute nothing).
pub fn sum_forces(loads: &[Load]) -> f64 {
    loads
        .iter()
        .map(|load| match *load {
            Load::PointForce { magnitude, .. } => magnitude,
            Load::LinearLoad {
                w_start,
                w_end,
                start,
                end,
            } => 0.5 * (w_start + w_end) * (end - start),
            _ => 0.0,
        })
        .sum()
}
