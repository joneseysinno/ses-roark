//! Phase 1: Roark 8.1, Table 8.1 (plain kernel).

use roark::beams::catalog::{RoarkEnd, RoarkLoad, table_8_1_case};
use roark::beams::state::{MOMENT, SHEAR};
use roark::beams::{Beam, BeamError, EndCondition, Load, sum_forces, sum_moments};

use crate::reference;
use crate::support::{assert_close, assert_matches_expected, end_residuals};

const L: f64 = 10.0;
const EI: f64 = 1.0e4;
const W: f64 = 2.5;
const A: f64 = 3.0;
const ENDS: [RoarkEnd; 6] = [
    RoarkEnd::A,
    RoarkEnd::B,
    RoarkEnd::C,
    RoarkEnd::D,
    RoarkEnd::E,
    RoarkEnd::F,
];

#[test]
fn concentrated_load_matches_references() {
    let expected = [
        reference::case_1a(W, A, L, EI),
        reference::case_1b(W, A, L, EI),
        reference::case_1c(W, A, L, EI),
        reference::case_1d(W, A, L, EI),
        reference::case_1e(W, A, L, EI),
        reference::case_1f(W, A, L, EI),
    ];
    for (end, expected) in ENDS.into_iter().zip(expected) {
        let solution = table_8_1_case(RoarkLoad::ConcentratedLoad { w: W, a: A }, end, L, EI)
            .solve()
            .unwrap();
        assert_matches_expected(&solution, &expected);
    }
}

#[test]
fn uniform_load_matches_references() {
    let expected = [
        reference::case_2a(W, L, EI),
        reference::case_2b(W, L, EI),
        reference::case_2c(W, L, EI),
        reference::case_2d(W, L, EI),
        reference::case_2e(W, L, EI),
        reference::case_2f(W, L, EI),
    ];
    for (end, expected) in ENDS.into_iter().zip(expected) {
        let solution = table_8_1_case(
            RoarkLoad::DistributedLoad {
                wa: W,
                wl: W,
                a: 0.0,
            },
            end,
            L,
            EI,
        )
        .solve()
        .unwrap();
        assert_matches_expected(&solution, &expected);
    }
}

#[test]
fn every_row_and_load_case_satisfies_statics_and_boundaries() {
    let loads = [
        RoarkLoad::ConcentratedLoad { w: W, a: 3.0 },
        RoarkLoad::DistributedLoad {
            wa: 1.0,
            wl: 4.0,
            a: 2.0,
        },
        RoarkLoad::ConcentratedMoment { mo: 7.0, a: 4.0 },
        RoarkLoad::AngularDeformation {
            theta_o: 0.01,
            a: 6.0,
        },
        RoarkLoad::LateralDisplacement {
            delta_o: 0.05,
            a: 5.0,
        },
        RoarkLoad::TemperatureDifferential {
            t1: 20.0,
            t2: 70.0,
            gamma: 1.2e-5,
            depth: 0.5,
            a: 3.0,
        },
    ];
    for end in ENDS {
        for load in loads {
            let beam = table_8_1_case(load, end, L, EI);
            let solution = beam.solve().unwrap();
            let (left, right) = (solution.state_at(0.0), solution.state_at(L));
            for residual in end_residuals(beam.left, &left)
                .into_iter()
                .chain(end_residuals(beam.right, &right))
            {
                assert_close(residual, 0.0, 0.0, 1e-9);
            }
            let (ra, rb) = (left[SHEAR], -right[SHEAR]);
            assert_close(ra + rb, sum_forces(&beam.loads), 0.0, 1e-9);
            assert_close(
                rb * L - left[MOMENT] + right[MOMENT] + sum_moments(&beam.loads, 0.0),
                0.0,
                0.0,
                1e-8,
            );
        }
    }
}

#[test]
fn maxwell_betti_reciprocity_holds_for_every_row() {
    for end in ENDS {
        let (left, right) = end.end_conditions();
        let deflection = |at: f64, x: f64| {
            Beam::new(
                L,
                EI,
                left,
                right,
                vec![Load::PointForce { magnitude: 1.0, at }],
            )
            .solve()
            .unwrap()
            .deflection(x)
        };
        assert_close(deflection(2.5, 7.0), deflection(7.0, 2.5), 1e-9, 1e-15);
    }
}

#[test]
fn responses_superpose() {
    let loads = [
        Load::PointForce {
            magnitude: 2.0,
            at: 3.0,
        },
        Load::LinearLoad {
            w_start: 1.0,
            w_end: 3.0,
            start: 1.0,
            end: 8.0,
        },
        Load::PointMoment {
            magnitude: 5.0,
            at: 6.5,
        },
    ];
    let solve = |loads: Vec<Load>| {
        Beam::new(L, EI, EndCondition::Fixed, EndCondition::Fixed, loads)
            .solve()
            .unwrap()
    };
    let combined = solve(loads.to_vec());
    let parts: Vec<_> = loads.iter().map(|load| solve(vec![*load])).collect();
    for x in [0.0, 2.0, 4.2, 6.5, 9.0, 10.0] {
        let y: f64 = parts.iter().map(|s| s.deflection(x)).sum();
        let m: f64 = parts.iter().map(|s| s.moment(x)).sum();
        assert_close(combined.deflection(x), y, 1e-9, 1e-14);
        assert_close(combined.moment(x), m, 1e-9, 1e-12);
    }
}

#[test]
fn roark_clockwise_moment_gives_negative_left_reaction() {
    let solution = table_8_1_case(
        RoarkLoad::ConcentratedMoment { mo: 7.0, a: 4.0 },
        RoarkEnd::E,
        L,
        EI,
    )
    .solve()
    .unwrap();
    let reactions = solution.reactions();
    assert_close(reactions.ra.unwrap(), -7.0 / L, 1e-12, 0.0);
    assert_close(reactions.rb.unwrap(), 7.0 / L, 1e-12, 0.0);
}

#[test]
fn temperature_differential_on_simple_and_fixed_spans() {
    let load = RoarkLoad::TemperatureDifferential {
        t1: 20.0,
        t2: 70.0,
        gamma: 1.2e-5,
        depth: 0.5,
        a: 0.0,
    };
    let curvature = 1.2e-5 * 50.0 / 0.5;
    let simple = table_8_1_case(load, RoarkEnd::E, L, EI).solve().unwrap();
    assert_close(
        simple.deflection(L / 2.0),
        -curvature * L * L / 8.0,
        1e-12,
        0.0,
    );
    assert_close(simple.moment(3.3), 0.0, 0.0, 1e-12);
    let fixed = table_8_1_case(load, RoarkEnd::D, L, EI).solve().unwrap();
    assert_close(fixed.moment(3.3), -EI * curvature, 1e-12, 0.0);
    assert_close(fixed.deflection(3.3), 0.0, 0.0, 1e-14);
}

#[test]
fn kinematic_actions_on_a_simple_span_are_rigid_body_motions() {
    let kink = table_8_1_case(
        RoarkLoad::AngularDeformation {
            theta_o: 0.01,
            a: 6.0,
        },
        RoarkEnd::E,
        L,
        EI,
    )
    .solve()
    .unwrap();
    assert_close(kink.deflection(6.0), -0.01 * 6.0 * 4.0 / L, 1e-12, 0.0);
    assert_close(kink.moment(5.0), 0.0, 0.0, 1e-12);
    let offset = table_8_1_case(
        RoarkLoad::LateralDisplacement {
            delta_o: 0.05,
            a: 5.0,
        },
        RoarkEnd::E,
        L,
        EI,
    )
    .solve()
    .unwrap();
    assert_close(offset.deflection(5.0 - 1e-12), -0.05 * 5.0 / L, 1e-9, 0.0);
    assert_close(offset.deflection(5.0), 0.05 - 0.05 * 5.0 / L, 1e-9, 0.0);
    assert_close(offset.moment(7.0), 0.0, 0.0, 1e-12);
}

#[test]
fn peak_responses_are_found() {
    let point = table_8_1_case(
        RoarkLoad::ConcentratedLoad { w: W, a: A },
        RoarkEnd::E,
        L,
        EI,
    )
    .solve()
    .unwrap();
    assert_close(point.max_moment(), W * A * (L - A) / L, 1e-9, 0.0);
    let uniform = table_8_1_case(
        RoarkLoad::DistributedLoad {
            wa: W,
            wl: W,
            a: 0.0,
        },
        RoarkEnd::E,
        L,
        EI,
    )
    .solve()
    .unwrap();
    assert_close(
        uniform.max_deflection(),
        -5.0 * W * L.powi(4) / (384.0 * EI),
        1e-9,
        0.0,
    );
    assert_close(uniform.max_moment(), W * L * L / 8.0, 1e-9, 0.0);
}

#[test]
fn peak_stations_of_a_triangular_load_and_a_propped_cantilever() {
    let rising = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Pinned,
        vec![Load::LinearLoad {
            w_start: 0.0,
            w_end: W,
            start: 0.0,
            end: L,
        }],
    )
    .solve()
    .unwrap();
    let moment = rising.peak_moment();
    assert_close(moment.x, L / 3.0_f64.sqrt(), 1e-6, 0.0);
    assert_close(moment.value, W * L * L / (9.0 * 3.0_f64.sqrt()), 1e-6, 0.0);
    assert_close(rising.peak_deflection().x / L, 0.5193, 0.0, 5.0e-5);
    let propped = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Fixed,
        vec![Load::uniform(W, 0.0, L)],
    )
    .solve()
    .unwrap();
    let hogging = propped.peak_moment();
    assert_close(hogging.x, L, 0.0, 1e-8);
    assert_close(hogging.value, -W * L * L / 8.0, 1e-6, 0.0);
}

#[test]
fn free_free_plain_beam_is_unstable() {
    let result = Beam::new(
        L,
        EI,
        EndCondition::Free,
        EndCondition::Free,
        vec![Load::PointForce {
            magnitude: 1.0,
            at: 5.0,
        }],
    )
    .solve();
    assert_eq!(result.unwrap_err(), BeamError::UnstableSupports);
}
