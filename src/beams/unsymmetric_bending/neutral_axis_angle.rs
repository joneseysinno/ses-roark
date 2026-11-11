use std::f64::consts::FRAC_PI_2;

use crate::plane_area::SectionProperties;

/// Neutral-axis angle (radians from `x`, in `(−π/2, π/2]`) where `A·y + B·x = 0`.
pub fn neutral_axis_angle(props: &SectionProperties, mx: f64, my: f64) -> f64 {
    let d = props.ix * props.iy - props.ixy * props.ixy;
    let a = (mx * props.iy - my * props.ixy) / d;
    let b = (my * props.ix - mx * props.ixy) / d;
    let angle = (-b).atan2(a);
    if angle > FRAC_PI_2 {
        angle - std::f64::consts::PI
    } else if angle <= -FRAC_PI_2 {
        angle + std::f64::consts::PI
    } else {
        angle
    }
}
