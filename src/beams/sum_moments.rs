use super::load::Load;

/// Net counterclockwise moment of the applied actions about station `about`.
pub fn sum_moments(loads: &[Load], about: f64) -> f64 {
    loads
        .iter()
        .map(|load| match *load {
            Load::PointForce { magnitude, at } => -magnitude * (at - about),
            Load::PointMoment { magnitude, .. } => magnitude,
            Load::LinearLoad {
                w_start,
                w_end,
                start,
                end,
            } => {
                let length = end - start;
                let offset = start - about;
                let slope = if length > 0.0 {
                    (w_end - w_start) / length
                } else {
                    0.0
                };
                -(w_start * (length * length / 2.0 + offset * length)
                    + slope * (length.powi(3) / 3.0 + offset * length * length / 2.0))
            }
            _ => 0.0,
        })
        .sum()
}
