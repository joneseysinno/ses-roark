use crate::plane_area::SectionProperties;

/// Generalized flexure stress at centroidal coordinates `(x, y)`:
/// `σ = −(A·y + B·x)` with `A = (M_x·I_y − M_y·I_xy)/D`,
/// `B = (M_y·I_x − M_x·I_xy)/D`, `D = I_x·I_y − I_xy²`.
pub fn bending_stress(props: &SectionProperties, mx: f64, my: f64, x: f64, y: f64) -> f64 {
    let d = props.ix * props.iy - props.ixy * props.ixy;
    let a = (mx * props.iy - my * props.ixy) / d;
    let b = (my * props.ix - mx * props.ixy) / d;
    -(a * y + b * x)
}
