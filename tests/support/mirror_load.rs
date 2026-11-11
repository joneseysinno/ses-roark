use roark::beams::Load;

/// End-for-end reflection of an applied action on a span of `length`.
///
/// A downward force stays downward. A couple, a slope, and a deflection offset
/// change sign because the beam axis reverses. A distributed action is clipped
/// to the reflected interval with its end intensities swapped.
pub fn mirror_load(load: Load, length: f64) -> Load {
    match load {
        Load::PointForce { magnitude, at } => Load::PointForce {
            magnitude,
            at: length - at,
        },
        Load::PointMoment { magnitude, at } => Load::PointMoment {
            magnitude: -magnitude,
            at: length - at,
        },
        Load::AngularDeformation { angle, at } => Load::AngularDeformation {
            angle,
            at: length - at,
        },
        Load::LateralDisplacement { offset, at } => Load::LateralDisplacement {
            offset: -offset,
            at: length - at,
        },
        Load::LinearLoad {
            w_start,
            w_end,
            start,
            end,
        } => Load::LinearLoad {
            w_start: w_end,
            w_end: w_start,
            start: length - end,
            end: length - start,
        },
        Load::ImposedCurvature {
            curvature,
            start,
            end,
        } => Load::ImposedCurvature {
            curvature,
            start: length - end,
            end: length - start,
        },
    }
}
