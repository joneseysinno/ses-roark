use crate::beams::load::Load;

/// Work of downward loads through a triangular virtual displacement that is
/// zero at both ends and `1` at the hinge `hinge`.
pub(crate) fn external_work(loads: &[Load], length: f64, hinge: f64) -> f64 {
    let shape = |x: f64| {
        if x <= hinge {
            x / hinge
        } else {
            (length - x) / (length - hinge)
        }
    };
    loads
        .iter()
        .map(|load| match *load {
            Load::PointForce { magnitude, at } => magnitude * shape(at),
            Load::LinearLoad {
                w_start,
                w_end,
                start,
                end,
            } => {
                if end <= start {
                    return 0.0;
                }
                let w = |x: f64| w_start + (w_end - w_start) * (x - start) / (end - start);
                let simpson = |a: f64, b: f64| {
                    let m = 0.5 * (a + b);
                    (b - a) / 6.0 * (w(a) * shape(a) + 4.0 * w(m) * shape(m) + w(b) * shape(b))
                };
                if hinge > start && hinge < end {
                    simpson(start, hinge) + simpson(hinge, end)
                } else {
                    simpson(start, end)
                }
            }
            _ => 0.0,
        })
        .sum()
}
