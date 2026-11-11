use nalgebra::SMatrix;

use super::Member;

/// Local stiffness of a prismatic frame member, dofs `[u1, v1, θ1, u2, v2, θ2]`.
pub(crate) fn member_stiffness(member: &Member, length: f64) -> SMatrix<f64, 6, 6> {
    let a = member.modulus * member.area / length;
    let b = member.modulus * member.inertia / length.powi(3);
    let l = length;
    SMatrix::<f64, 6, 6>::from_row_slice(&[
        a,
        0.0,
        0.0,
        -a,
        0.0,
        0.0,
        0.0,
        12.0 * b,
        6.0 * b * l,
        0.0,
        -12.0 * b,
        6.0 * b * l,
        0.0,
        6.0 * b * l,
        4.0 * b * l * l,
        0.0,
        -6.0 * b * l,
        2.0 * b * l * l,
        -a,
        0.0,
        0.0,
        a,
        0.0,
        0.0,
        0.0,
        -12.0 * b,
        -6.0 * b * l,
        0.0,
        12.0 * b,
        -6.0 * b * l,
        0.0,
        6.0 * b * l,
        2.0 * b * l * l,
        0.0,
        -6.0 * b * l,
        4.0 * b * l * l,
    ])
}
