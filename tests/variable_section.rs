//! Phase 5: Roark 8.8 variable section (Table 8.11) and 8.9 slotted beams.

use roark::beams::variable_section::{
    slot_secondary_moments, slotted_stiffness, taper_coefficients,
};
use roark::beams::{Beam, BeamError, EndCondition, Load, Stiffness};

use crate::support::assert_close;

const L: f64 = 10.0;
const EI: f64 = 1.0e4;

#[test]
fn single_segment_piecewise_matches_plain() {
    let loads = vec![
        Load::PointForce {
            magnitude: 3.0,
            at: 2.0,
        },
        Load::LinearLoad {
            w_start: 1.0,
            w_end: 2.0,
            start: 3.0,
            end: 9.0,
        },
        Load::PointMoment {
            magnitude: 4.0,
            at: 6.0,
        },
        Load::ImposedCurvature {
            curvature: 1e-3,
            start: 1.0,
            end: 5.0,
        },
    ];
    let plain = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Pinned,
        loads.clone(),
    )
    .solve()
    .unwrap();
    let mut variable = Beam::new(L, EI, EndCondition::Fixed, EndCondition::Pinned, loads);
    variable.stiffness = Stiffness::Piecewise(vec![(L, EI)]);
    let variable = variable.solve().unwrap();
    for x in [0.5, 2.0, 4.4, 6.0, 8.8, L] {
        assert_close(variable.deflection(x), plain.deflection(x), 1e-12, 1e-15);
        assert_close(variable.moment(x), plain.moment(x), 1e-12, 1e-12);
    }
}

#[test]
fn width_tapered_cantilever_has_constant_curvature() {
    let w = 2.0;
    let mut beam = Beam::new(
        L,
        EI,
        EndCondition::Free,
        EndCondition::Fixed,
        vec![Load::PointForce {
            magnitude: w,
            at: 0.0,
        }],
    );
    beam.stiffness = Stiffness::Tapered {
        ei_start: 0.0,
        ei_end: EI,
        exponent: 1.0,
    };
    let solution = beam.solve().unwrap();
    assert_close(
        solution.deflection(0.0),
        -w * L.powi(3) / (2.0 * EI),
        1e-10,
        0.0,
    );
    assert_close(solution.slope(0.0), w * L * L / EI, 1e-10, 0.0);
}

#[test]
fn width_tapered_cantilever_loaded_at_the_right_tip() {
    let w = 2.0;
    let mut beam = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Free,
        vec![Load::PointForce {
            magnitude: w,
            at: L,
        }],
    );
    beam.stiffness = Stiffness::Tapered {
        ei_start: EI,
        ei_end: 0.0,
        exponent: 1.0,
    };
    let solution = beam.solve().unwrap();
    assert_close(
        solution.deflection(L),
        -w * L.powi(3) / (2.0 * EI),
        1e-10,
        0.0,
    );
    assert_close(solution.slope(L), -w * L * L / EI, 1e-10, 0.0);
}

#[test]
fn quadratic_taper_under_uniform_load_matches_the_free_end() {
    let mut beam = Beam::new(
        L,
        EI,
        EndCondition::Free,
        EndCondition::Fixed,
        vec![Load::uniform(1.0, 0.0, L)],
    );
    beam.stiffness = Stiffness::Tapered {
        ei_start: 0.0,
        ei_end: EI,
        exponent: 2.0,
    };
    let solution = beam.solve().unwrap();
    assert_close(
        solution.deflection(0.0),
        -L.powi(4) / (4.0 * EI),
        1e-10,
        0.0,
    );
}

#[test]
fn singular_tapers_are_rejected() {
    let reject = |left, right, stiffness: Stiffness, loads| {
        let mut beam = Beam::new(L, EI, left, right, loads);
        beam.stiffness = stiffness;
        assert!(
            matches!(beam.solve(), Err(BeamError::InvalidInput(_))),
            "expected InvalidInput for {beam:?}"
        );
    };
    reject(
        EndCondition::Free,
        EndCondition::Fixed,
        Stiffness::Tapered {
            ei_start: 0.0,
            ei_end: EI,
            exponent: 3.0,
        },
        vec![Load::PointForce {
            magnitude: 1.0,
            at: 0.0,
        }],
    );
    reject(
        EndCondition::Free,
        EndCondition::Fixed,
        Stiffness::Tapered {
            ei_start: 0.0,
            ei_end: EI,
            exponent: 3.0,
        },
        vec![Load::uniform(1.0, 0.0, L)],
    );
    reject(
        EndCondition::Free,
        EndCondition::Fixed,
        Stiffness::Tapered {
            ei_start: 0.0,
            ei_end: EI,
            exponent: 1.0,
        },
        vec![Load::PointMoment {
            magnitude: 1.0,
            at: 0.0,
        }],
    );
    reject(
        EndCondition::Fixed,
        EndCondition::Free,
        Stiffness::Tapered {
            ei_start: 0.0,
            ei_end: EI,
            exponent: 1.0,
        },
        vec![Load::PointForce {
            magnitude: 1.0,
            at: L,
        }],
    );
}

#[test]
fn pinned_zero_end_matches_a_vanishing_stiffness() {
    let mut singular = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Pinned,
        vec![Load::PointForce {
            magnitude: 1.0,
            at: L / 2.0,
        }],
    );
    singular.stiffness = Stiffness::Tapered {
        ei_start: 0.0,
        ei_end: EI,
        exponent: 1.0,
    };
    let mut tiny = singular.clone();
    tiny.stiffness = Stiffness::Tapered {
        ei_start: 1e-9 * EI,
        ei_end: EI,
        exponent: 1.0,
    };
    let singular = singular.solve().unwrap();
    let tiny = tiny.solve().unwrap();
    assert_close(
        singular.deflection(L / 2.0),
        tiny.deflection(L / 2.0),
        1e-6,
        0.0,
    );
}

#[test]
fn stepped_cantilever_matches_hand_integration() {
    let w = 2.0;
    let mut beam = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Free,
        vec![Load::PointForce {
            magnitude: w,
            at: L,
        }],
    );
    beam.stiffness = Stiffness::Piecewise(vec![(L / 2.0, 2.0 * EI), (L, EI)]);
    let solution = beam.solve().unwrap();
    assert_close(
        solution.deflection(L),
        -3.0 * w * L.powi(3) / (16.0 * EI),
        1e-12,
        0.0,
    );
}

#[test]
fn uniform_taper_coefficients_are_unity() {
    let mut beam = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Fixed,
        vec![Load::uniform(1.0, 0.0, L)],
    );
    beam.stiffness = Stiffness::Tapered {
        ei_start: EI,
        ei_end: EI,
        exponent: 3.0,
    };
    let ratios = taper_coefficients(&beam).unwrap();
    assert_close(ratios.max_deflection, 1.0, 1e-9, 0.0);
    assert_close(ratios.right_moment.unwrap(), 1.0, 1e-9, 0.0);
}

#[test]
fn deepening_taper_stiffens_the_beam() {
    let mut beam = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Pinned,
        vec![Load::uniform(1.0, 0.0, L)],
    );
    beam.stiffness = Stiffness::Tapered {
        ei_start: EI,
        ei_end: 8.0 * EI,
        exponent: 3.0,
    };
    let ratios = taper_coefficients(&beam).unwrap();
    assert!(ratios.max_deflection < 1.0);
    assert_close(ratios.max_moment, 1.0, 1e-9, 0.0);
}

#[test]
fn slotted_beam_helpers() {
    assert_eq!(
        slotted_stiffness(EI, 0.25 * EI, 4.0, 6.0, L),
        Stiffness::Piecewise(vec![(4.0, EI), (6.0, 0.25 * EI), (L, EI)])
    );
    let (top, bottom) = slot_secondary_moments(10.0, 2.0, 1.0, 1.0);
    assert_close(top, 10.0 * 2.0 / 4.0, 1e-12, 0.0);
    assert_close(bottom, top, 1e-12, 0.0);
    let mut beam = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Pinned,
        vec![Load::PointForce {
            magnitude: 1.0,
            at: 5.0,
        }],
    );
    let solid = beam.solve().unwrap().deflection(5.0);
    beam.stiffness = slotted_stiffness(EI, 0.25 * EI, 4.0, 6.0, L);
    assert!(beam.solve().unwrap().deflection(5.0) < solid);
}
