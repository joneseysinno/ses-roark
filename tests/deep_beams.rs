//! Phase 6: Roark 8.10, beams of relatively great depth.

use roark::beams::deep_beams::{
    form_factor_layered, shear_rigidity, uniform_load_stress_correction,
};
use roark::beams::{Beam, EndCondition, Load};

use crate::support::assert_close;

const L: f64 = 4.0;
const EI: f64 = 1.0e4;
const S: f64 = 2.0e4;

#[test]
fn shear_deflection_adds_to_bending_on_a_simple_span() {
    let p = 3.0;
    let mut beam = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Pinned,
        vec![Load::PointForce {
            magnitude: p,
            at: L / 2.0,
        }],
    );
    beam.shear_rigidity = Some(S);
    let solution = beam.solve().unwrap();
    let expected = -p * L.powi(3) / (48.0 * EI) - p * L / (4.0 * S);
    assert_close(solution.deflection(L / 2.0), expected, 1e-12, 0.0);
}

#[test]
fn shear_flexible_cantilever_and_fixed_beam() {
    let p = 3.0;
    let mut cantilever = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Free,
        vec![Load::PointForce {
            magnitude: p,
            at: L,
        }],
    );
    cantilever.shear_rigidity = Some(S);
    let tip = cantilever.solve().unwrap().deflection(L);
    assert_close(tip, -p * L.powi(3) / (3.0 * EI) - p * L / S, 1e-12, 0.0);

    let w = 2.0;
    let mut fixed = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Fixed,
        vec![Load::uniform(w, 0.0, L)],
    );
    fixed.shear_rigidity = Some(S);
    let solution = fixed.solve().unwrap();
    assert_close(solution.moment(0.0), -w * L * L / 12.0, 1e-12, 0.0);
    assert_close(
        solution.deflection(L / 2.0),
        -w * L.powi(4) / (384.0 * EI) - w * L * L / (8.0 * S),
        1e-12,
        0.0,
    );
}

#[test]
fn rectangle_form_factor_is_six_fifths() {
    assert_close(form_factor_layered(&[(2.0, 3.0)]), 1.2, 1e-12, 0.0);
    assert_close(
        form_factor_layered(&[(2.0, 1.0), (2.0, 2.0)]),
        1.2,
        1e-12,
        0.0,
    );
}

#[test]
fn i_section_form_factor_exceeds_rectangle() {
    let f = form_factor_layered(&[(6.0, 1.0), (0.5, 8.0), (6.0, 1.0)]);
    let area = 6.0 * 2.0 + 0.5 * 8.0;
    assert!(f > 1.2);
    assert!(f < 1.5 * area / (0.5 * 10.0));
    assert_close(shear_rigidity(8.0e4, area, f), 8.0e4 * area / f, 1e-12, 0.0);
}

#[test]
fn uniform_load_correction_is_w_over_five_b() {
    assert_close(uniform_load_stress_correction(10.0, 1.0), 2.0, 1e-12, 0.0);
    assert_close(uniform_load_stress_correction(10.0, 4.0), 0.5, 1e-12, 0.0);
}
