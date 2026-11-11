use super::end_condition::{EndCondition, Side};
use super::state::{DEFLECTION, MOMENT, SHEAR, SLOPE, StateRow};

/// Two homogeneous boundary equations `row · s = 0` for one end.
///
/// Springs act against the motion: at the left end `V = −k_v·y` and
/// `M = k_θ·θ`; at the right end `V = k_v·y` and `M = −k_θ·θ`.
pub(crate) fn end_rows(end: EndCondition, side: Side) -> [StateRow; 2] {
    let unit = |i: usize| StateRow::from_fn(|_, j| if j == i { 1.0 } else { 0.0 });
    let sign = match side {
        Side::Left => 1.0,
        Side::Right => -1.0,
    };
    match end {
        EndCondition::Free => [unit(MOMENT), unit(SHEAR)],
        EndCondition::Guided => [unit(SLOPE), unit(SHEAR)],
        EndCondition::Pinned => [unit(DEFLECTION), unit(MOMENT)],
        EndCondition::Fixed => [unit(DEFLECTION), unit(SLOPE)],
        EndCondition::Elastic {
            vertical,
            rotational,
        } => {
            let translational = if vertical.is_infinite() {
                unit(DEFLECTION)
            } else {
                StateRow::new(sign * vertical, 0.0, 0.0, 1.0)
            };
            let rotary = if rotational.is_infinite() {
                unit(SLOPE)
            } else {
                StateRow::new(0.0, -sign * rotational, 1.0, 0.0)
            };
            [translational, rotary]
        }
    }
}
