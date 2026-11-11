//! Independent oracles for all 36 Table 8.1 combinations.
//!
//! The quadrature check compares the polynomial kernel with the variable-section
//! kernel on a two-segment beam of the same `EI`. For cases 4, 5, and 6 that
//! kernel reuses the plain kinematic load terms, so only the transfer matrix
//! and the boundary solve are independent there; the mirror check and the
//! closed forms cover the load terms.

use roark::beams::catalog::{RoarkEnd, RoarkLoad, table_8_1_case};
use roark::beams::{Beam, BeamSolution, EndCondition, Load, State, Stiffness};

use crate::support::{assert_close, mirror_load};

const L: f64 = 10.0;
const EI: f64 = 1.0e4;

const ENDS: [RoarkEnd; 6] = [
    RoarkEnd::A,
    RoarkEnd::B,
    RoarkEnd::C,
    RoarkEnd::D,
    RoarkEnd::E,
    RoarkEnd::F,
];

const COMPONENTS: [&str; 4] = ["y", "theta", "M", "V"];

fn table_loads() -> [RoarkLoad; 6] {
    [
        RoarkLoad::ConcentratedLoad { w: 2.5, a: 3.0 },
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
    ]
}

/// Peak `|component|` of `solution` on 21 stations from 0 to `L`.
fn component_peaks(solution: &BeamSolution) -> [f64; 4] {
    let mut peaks = [0.0; 4];
    for i in 0..=20 {
        let state = solution.state_at(i as f64 * L / 20.0);
        for (peak, value) in peaks.iter_mut().zip(state.iter()) {
            *peak = f64::max(*peak, value.abs());
        }
    }
    peaks
}

fn assert_normalized(label: &str, actual: State, expected: State, peaks: [f64; 4]) {
    for (c, name) in COMPONENTS.into_iter().enumerate() {
        if peaks[c] == 0.0 {
            continue;
        }
        let difference = (actual[c] - expected[c]).abs() / peaks[c];
        assert!(
            difference <= 1e-11,
            "{label} {name}: normalized difference {difference:e} \
             (actual {:e}, expected {:e}, peak {:e})",
            actual[c],
            expected[c],
            peaks[c]
        );
    }
}

#[test]
fn quadrature_oracle_matches_plain_kernel() {
    let stations = [0.0, 1.3, 2.9, 4.5, 5.5, 7.7, 9.1, L];
    for end in ENDS {
        for load in table_loads() {
            let plain_beam = table_8_1_case(load, end, L, EI);
            let mut variable_beam = plain_beam.clone();
            variable_beam.stiffness = Stiffness::Piecewise(vec![(0.37 * L, EI), (L, EI)]);
            let plain = plain_beam.solve().unwrap();
            let variable = variable_beam.solve().unwrap();
            let peaks = component_peaks(&plain);
            for x in stations {
                assert_normalized(
                    &format!("{end:?} {load:?} at x = {x}"),
                    variable.state_at(x),
                    plain.state_at(x),
                    peaks,
                );
            }
        }
    }
}

#[test]
fn mirror_symmetry_reflects_the_solution() {
    let stations = [0.4, 1.3, 2.9, 4.5, 5.5, 7.7, 9.1, 9.6];
    for end in ENDS {
        for load in table_loads() {
            let beam = table_8_1_case(load, end, L, EI);
            let mirrored = Beam::new(
                L,
                EI,
                beam.right,
                beam.left,
                vec![mirror_load(beam.loads[0], L)],
            );
            let solution = beam.solve().unwrap();
            let mirror = mirrored.solve().unwrap();
            let peaks = component_peaks(&solution);
            for x in stations {
                let reflected = mirror.state_at(L - x);
                let expected = State::new(reflected[0], -reflected[1], reflected[2], -reflected[3]);
                assert_normalized(
                    &format!("{end:?} {load:?} at x = {x}"),
                    solution.state_at(x),
                    expected,
                    peaks,
                );
            }
        }
    }
}

#[test]
fn fixed_fixed_midspan_moment_matches_closed_form() {
    let m0 = 12.0;
    let reactions = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Fixed,
        vec![Load::PointMoment {
            magnitude: m0,
            at: L / 2.0,
        }],
    )
    .solve()
    .unwrap()
    .reactions();
    assert_close(reactions.ma.unwrap(), -m0 / 4.0, 1e-12, 0.0);
    assert_close(reactions.mb.unwrap(), m0 / 4.0, 1e-12, 0.0);
    assert_close(reactions.ra.unwrap(), 3.0 * m0 / (2.0 * L), 1e-12, 0.0);
    assert_close(reactions.rb.unwrap(), -3.0 * m0 / (2.0 * L), 1e-12, 0.0);
}

#[test]
fn pinned_pinned_point_moment_matches_closed_form() {
    let m0 = 7.0;
    let a = 3.0;
    let solution = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Pinned,
        vec![Load::PointMoment {
            magnitude: m0,
            at: a,
        }],
    )
    .solve()
    .unwrap();
    let reactions = solution.reactions();
    assert_close(reactions.ra.unwrap(), m0 / L, 1e-12, 0.0);
    assert_close(reactions.rb.unwrap(), -m0 / L, 1e-12, 0.0);
    assert_close(
        solution.deflection(a),
        m0 * a * (L - a) * (L - 2.0 * a) / (3.0 * EI * L),
        1e-12,
        0.0,
    );
}

#[test]
fn fixed_fixed_midspan_kink_is_uniform_moment() {
    let theta0 = 0.01;
    let solution = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Fixed,
        vec![Load::AngularDeformation {
            angle: theta0,
            at: L / 2.0,
        }],
    )
    .solve()
    .unwrap();
    for x in [0.0, 1.3, L / 2.0, 7.7, L] {
        assert_close(solution.moment(x), -EI * theta0 / L, 1e-12, 0.0);
        assert_close(solution.shear(x), 0.0, 0.0, 1e-12);
    }
}

#[test]
fn fixed_fixed_midspan_offset_matches_closed_form() {
    let delta0 = 0.05;
    let reactions = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Fixed,
        vec![Load::LateralDisplacement {
            offset: delta0,
            at: L / 2.0,
        }],
    )
    .solve()
    .unwrap()
    .reactions();
    assert_close(
        reactions.ra.unwrap(),
        12.0 * EI * delta0 / L.powi(3),
        1e-12,
        0.0,
    );
    assert_close(
        reactions.ma.unwrap(),
        -6.0 * EI * delta0 / L.powi(2),
        1e-12,
        0.0,
    );
    assert_close(
        reactions.mb.unwrap(),
        6.0 * EI * delta0 / L.powi(2),
        1e-12,
        0.0,
    );
}
