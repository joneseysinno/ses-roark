//! Phase 9: Appendix A section properties, 8.12 shear lag, 8.13 thin webs,
//! 8.14 unsymmetric bending and flexural center.

use std::f64::consts::{FRAC_PI_4, PI};

use roark::beams::flexural_center::{ThinWalledSection, shear_center};
use roark::beams::shear_lag::{compression_flange_effective_width, effective_width};
use roark::beams::thin_webs::{
    diagonal_tension_stress, flange_force, shear_buckling_coefficient, shear_buckling_stress,
    stiffener_force,
};
use roark::beams::unsymmetric_bending::{bending_stress, neutral_axis_angle};
use roark::plane_area::{Polygon, elastic_section_modulus, plastic_modulus, section_properties};

use crate::support::{assert_close, rotated_rectangle};

#[test]
fn rectangle_properties() {
    let section = [Polygon::rectangle(1.0, 2.0, 3.0, 5.0)];
    let props = section_properties(&section);
    assert_close(props.area, 15.0, 1e-12, 0.0);
    assert_close(props.centroid.0, 2.5, 1e-12, 0.0);
    assert_close(props.centroid.1, 4.5, 1e-12, 0.0);
    assert_close(props.ix, 3.0 * 125.0 / 12.0, 1e-12, 0.0);
    assert_close(props.iy, 5.0 * 27.0 / 12.0, 1e-12, 0.0);
    assert_close(props.ixy, 0.0, 0.0, 1e-10);
    assert_close(
        elastic_section_modulus(&section),
        3.0 * 25.0 / 6.0,
        1e-12,
        0.0,
    );
    assert_close(plastic_modulus(&section), 3.0 * 25.0 / 4.0, 1e-10, 0.0);
}

#[test]
fn holes_subtract() {
    let outer = Polygon::rectangle(0.0, 0.0, 4.0, 4.0);
    let mut inner = Polygon::rectangle(1.0, 1.0, 2.0, 2.0);
    inner.vertices.reverse();
    let props = section_properties(&[outer, inner]);
    assert_close(props.area, 12.0, 1e-12, 0.0);
    assert_close(props.ix, (4.0 * 64.0 - 2.0 * 8.0) / 12.0, 1e-12, 0.0);
}

#[test]
fn rotated_rectangle_principal_axes() {
    let angle = PI / 6.0;
    let props = section_properties(&[rotated_rectangle(4.0, 1.0, angle)]);
    let axes = props.principal_axes();
    assert_close(axes.i_max, 64.0 / 12.0, 1e-12, 0.0);
    assert_close(axes.i_min, 4.0 / 12.0, 1e-12, 0.0);
    let offset = (axes.angle - (angle + PI / 2.0)).rem_euclid(PI);
    assert!(offset < 1e-12 || PI - offset < 1e-12);
}

#[test]
fn unsymmetric_bending_matches_principal_decomposition() {
    let angle = PI / 6.0;
    let props = section_properties(&[rotated_rectangle(4.0, 1.0, angle)]);
    let (mx, my) = (10.0, -3.0);
    let (s, c) = angle.sin_cos();
    let (along, across) = (16.0 / 3.0, 1.0 / 3.0);
    let m_xi = c * my + s * mx;
    let m_eta = -s * my + c * mx;
    let (x, y) = (2.0 * c - 0.5 * s, 2.0 * s + 0.5 * c);
    let (xi, eta) = (x * c + y * s, -x * s + y * c);
    let expected = -(m_eta / across * eta + m_xi / along * xi);
    assert_close(bending_stress(&props, mx, my, x, y), expected, 1e-12, 0.0);
    let na = neutral_axis_angle(&props, mx, my);
    assert_close(
        bending_stress(&props, mx, my, na.cos(), na.sin()),
        0.0,
        0.0,
        1e-10,
    );
}

#[test]
fn symmetric_section_neutral_axis_is_horizontal() {
    let props = section_properties(&[Polygon::rectangle(0.0, 0.0, 2.0, 6.0)]);
    assert_close(neutral_axis_angle(&props, 5.0, 0.0), 0.0, 0.0, 1e-14);
    assert_close(
        bending_stress(&props, 5.0, 0.0, 0.0, 3.0),
        -5.0 * 3.0 / 36.0,
        1e-12,
        0.0,
    );
}

#[test]
fn channel_shear_center() {
    let (b, h, tf, tw) = (3.0, 8.0, 0.4, 0.3);
    let (x, y) = shear_center(&ThinWalledSection::channel(b, h, tf, tw)).unwrap();
    assert_close(x, -3.0 * b * b * tf / (h * tw + 6.0 * b * tf), 1e-10, 0.0);
    assert_close(y, 0.0, 0.0, 1e-10);
}

#[test]
fn i_section_shear_centers() {
    let (x, y) =
        shear_center(&ThinWalledSection::i_section(4.0, 4.0, 10.0, 0.5, 0.5, 0.3)).unwrap();
    assert_close(x, 0.0, 0.0, 1e-10);
    assert_close(y, 0.0, 0.0, 1e-10);
    let (x, y) =
        shear_center(&ThinWalledSection::i_section(4.0, 6.0, 10.0, 0.5, 0.5, 0.3)).unwrap();
    let (top, bottom) = (0.5 * 64.0, 0.5 * 216.0);
    assert_close(x, 0.0, 0.0, 1e-10);
    assert_close(y, 5.0 - 10.0 * bottom / (top + bottom), 1e-10, 0.0);
}

#[test]
fn angle_and_tee_shear_centers_sit_at_the_wall_junction() {
    let (x, y) = shear_center(&ThinWalledSection::angle(4.0, 3.0, 0.25)).unwrap();
    assert_close(x, 0.0, 0.0, 1e-10);
    assert_close(y, 0.0, 0.0, 1e-10);
    let (x, y) = shear_center(&ThinWalledSection::tee(5.0, 4.0, 0.5, 0.3)).unwrap();
    assert_close(x, 0.0, 0.0, 1e-10);
    assert_close(y, 0.0, 0.0, 1e-10);
}

#[test]
fn i_section_plastic_modulus() {
    let (b, tf, tw, h) = (6.0, 1.0, 0.5, 10.0);
    let section = [
        Polygon::rectangle(-b / 2.0, 0.0, b, tf),
        Polygon::rectangle(-tw / 2.0, tf, tw, h - 2.0 * tf),
        Polygon::rectangle(-b / 2.0, h - tf, b, tf),
    ];
    let expected = b * tf * (h - tf) + tw * (h - 2.0 * tf).powi(2) / 4.0;
    assert_close(plastic_modulus(&section), expected, 1e-10, 0.0);
}

#[test]
fn shear_lag_and_thin_web_helpers() {
    assert_close(effective_width(10.0, 0.8), 8.0, 1e-12, 0.0);
    assert_close(
        compression_flange_effective_width(0.1, 2.0e5, 200.0),
        1.9 * 0.1 * 1000.0_f64.sqrt(),
        1e-12,
        0.0,
    );
    assert_close(shear_buckling_coefficient(2.0, 2.0), 9.35, 1e-12, 0.0);
    assert_close(
        shear_buckling_stress(9.35, 2.0e5, 0.3, 0.01, 1.0),
        9.35 * PI * PI * 2.0e5 / (12.0 * 0.91) * 1e-4,
        1e-12,
        0.0,
    );
    assert_close(diagonal_tension_stress(50.0, FRAC_PI_4), 100.0, 1e-12, 0.0);
    assert_close(
        stiffener_force(50.0, 0.01, 0.5, FRAC_PI_4),
        0.25,
        1e-12,
        0.0,
    );
    assert_close(flange_force(80.0, FRAC_PI_4), 40.0, 1e-12, 0.0);
}
