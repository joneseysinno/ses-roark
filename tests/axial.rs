//! Phase 4: Roark 8.7, axial plus transverse loading (Tables 8.7-8.10).

use roark::beams::axial::{
    AxialSense, amplification, critical_load, restrained_ends, table_8_8_values,
};
use roark::beams::{Beam, BeamError, BeamSolution, EndCondition, Kernel, Load, State, sum_forces};

use crate::support::{assert_close, end_residuals, euler_load, mirror_load, uniform_simple_beam};

const L: f64 = 10.0;
const EI: f64 = 1.0e4;
const W: f64 = 2.0;

#[test]
fn small_axial_load_converges_to_plain() {
    let loads = vec![
        Load::PointForce {
            magnitude: 3.0,
            at: 2.0,
        },
        Load::uniform(1.0, 4.0, 9.0),
    ];
    let mut compressed = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Pinned,
        loads.clone(),
    );
    compressed.kernel = Kernel::AxialCompression {
        force: 1e-6 * euler_load(EI, L),
    };
    let plain = Beam::new(L, EI, EndCondition::Fixed, EndCondition::Pinned, loads)
        .solve()
        .unwrap();
    let compressed = compressed.solve().unwrap();
    for x in [1.0, 4.0, 8.0] {
        assert_close(compressed.deflection(x), plain.deflection(x), 1e-5, 0.0);
        assert_close(compressed.moment(x), plain.moment(x), 1e-5, 0.0);
    }
}

#[test]
fn simply_supported_beam_column_matches_closed_form() {
    let force = 0.4 * euler_load(EI, L);
    let k = (force / EI).sqrt();
    let u = k * L / 2.0;
    let solution = uniform_simple_beam(L, EI, W, Kernel::AxialCompression { force })
        .solve()
        .unwrap();
    let y = -(5.0 * W * L.powi(4) / (384.0 * EI)) * 12.0 * (2.0 / u.cos() - 2.0 - u * u)
        / (5.0 * u.powi(4));
    assert_close(solution.deflection(L / 2.0), y, 1e-9, 0.0);
    assert_close(
        solution.moment(L / 2.0),
        W / (k * k) * (1.0 / u.cos() - 1.0),
        1e-9,
        0.0,
    );

    let tension = uniform_simple_beam(L, EI, W, Kernel::AxialTension { force })
        .solve()
        .unwrap();
    let y = -(5.0 * W * L.powi(4) / (384.0 * EI)) * 12.0 * (2.0 / u.cosh() - 2.0 + u * u)
        / (5.0 * u.powi(4));
    assert_close(tension.deflection(L / 2.0), y, 1e-9, 0.0);
    assert_close(
        tension.moment(L / 2.0),
        W / (k * k) * (1.0 - 1.0 / u.cosh()),
        1e-9,
        0.0,
    );
}

#[test]
fn critical_loads_match_euler() {
    let critical = |left, right| critical_load(&Beam::new(L, EI, left, right, Vec::new())).unwrap();
    use EndCondition::{Fixed, Free, Pinned};
    assert_close(critical(Pinned, Pinned), euler_load(EI, L), 1e-9, 0.0);
    assert_close(critical(Fixed, Free), euler_load(EI, L) / 4.0, 1e-9, 0.0);
    assert_close(critical(Fixed, Fixed), 4.0 * euler_load(EI, L), 1e-9, 0.0);
    let root: f64 = 4.493_409_457_909_064;
    assert_close(
        critical(Pinned, Fixed),
        root * root * EI / (L * L),
        1e-9,
        0.0,
    );
}

#[test]
fn compression_beyond_critical_is_rejected() {
    let result = uniform_simple_beam(
        L,
        EI,
        W,
        Kernel::AxialCompression {
            force: 1.01 * euler_load(EI, L),
        },
    )
    .solve();
    assert!(matches!(
        result,
        Err(BeamError::AtOrAboveCriticalLoad { .. })
    ));
}

#[test]
fn amplification_approaches_the_moment_magnifier() {
    let alpha = 0.05;
    let ratios = amplification(&uniform_simple_beam(
        L,
        EI,
        W,
        Kernel::AxialCompression {
            force: alpha * euler_load(EI, L),
        },
    ))
    .unwrap();
    assert_close(ratios.max_deflection, 1.0 / (1.0 - alpha), 1e-2, 0.0);
    assert_close(ratios.max_moment, 1.0 / (1.0 - alpha), 1e-2, 0.0);
    assert_close(ratios.left_reaction.unwrap(), 1.0, 1e-12, 0.0);
}

#[test]
fn negative_compression_is_tension() {
    let a = uniform_simple_beam(L, EI, W, Kernel::AxialCompression { force: -300.0 })
        .solve()
        .unwrap();
    let b = uniform_simple_beam(L, EI, W, Kernel::AxialTension { force: 300.0 })
        .solve()
        .unwrap();
    for x in [1.0, 5.0, 8.0] {
        assert_close(a.deflection(x), b.deflection(x), 1e-14, 0.0);
    }
}

#[test]
fn table_8_8_values_match_definitions() {
    let c = table_8_8_values(0.8, AxialSense::Compression);
    assert_close(c[2], 1.0 - 0.8_f64.cos(), 1e-15, 0.0);
    assert_close(c[3], 0.8 - 0.8_f64.sin(), 1e-15, 0.0);
    let t = table_8_8_values(0.8, AxialSense::Tension);
    assert_close(t[0] * t[0] - t[1] * t[1], 1.0, 1e-14, 0.0);
}

#[test]
fn restrained_ends_induce_consistent_tension() {
    let axial_rigidity = 1.0e5;
    let beam = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Pinned,
        vec![Load::uniform(50.0, 0.0, L)],
    );
    let restrained = restrained_ends(&beam, axial_rigidity).unwrap();
    let n = 4000;
    let h = L / n as f64;
    let integral: f64 = (0..=n)
        .map(|i| {
            let weight = if i == 0 || i == n {
                1.0
            } else if i % 2 == 1 {
                4.0
            } else {
                2.0
            };
            weight * restrained.solution.slope(i as f64 * h).powi(2)
        })
        .sum::<f64>()
        * h
        / 3.0;
    assert_close(
        restrained.tension,
        axial_rigidity / (2.0 * L) * integral,
        1e-8,
        0.0,
    );
    let plain = beam.solve().unwrap();
    assert!(restrained.solution.max_deflection().abs() < plain.max_deflection().abs());
    assert!(restrained.tension > 0.0);
}

fn steel_strip(load: f64, stiffness_factor: f64) -> (Beam, f64) {
    let modulus = 29.0e6;
    let width = 1.0;
    let thickness = 0.25;
    let length = 40.0;
    let axial = modulus * width * thickness;
    let beam = Beam::new(
        length,
        stiffness_factor * modulus * width * thickness.powi(3) / 12.0,
        EndCondition::Pinned,
        EndCondition::Pinned,
        vec![Load::uniform(load, 0.0, length)],
    );
    (beam, axial)
}

fn simpson_slope_squared(solution: &BeamSolution, length: f64) -> f64 {
    let n = 4000;
    let h = length / n as f64;
    (0..=n)
        .map(|i| {
            let weight = if i == 0 || i == n {
                1.0
            } else if i % 2 == 1 {
                4.0
            } else {
                2.0
            };
            weight * solution.slope(i as f64 * h).powi(2)
        })
        .sum::<f64>()
        * h
        / 3.0
}

#[test]
fn restrained_steel_strip_follows_the_membrane_curve() {
    let length = 40.0;
    let expected = [(0.5, 350.3), (1.0, 634.2), (2.0, 1090.4), (2.5, 1288.9)];
    let mut tensions = Vec::new();
    for (load, reference) in expected {
        let (beam, axial) = steel_strip(load, 1.0);
        let restrained = restrained_ends(&beam, axial).unwrap();
        assert_close(restrained.tension, reference, 1e-3, 0.0);
        tensions.push((load, axial, restrained.tension));
    }
    for load in [3.0, 4.0, 10.0] {
        let (beam, axial) = steel_strip(load, 1.0);
        let restrained = restrained_ends(&beam, axial).unwrap();
        let integral = simpson_slope_squared(&restrained.solution, length);
        assert_close(
            restrained.tension,
            axial / (2.0 * length) * integral,
            1e-6,
            0.0,
        );
        tensions.push((load, axial, restrained.tension));
    }
    for pair in tensions.windows(2) {
        assert!(
            pair[1].2 > pair[0].2,
            "tension decreased from w = {} to w = {}",
            pair[0].0,
            pair[1].0
        );
    }
    for (load, axial, tension) in tensions {
        let cable = (axial * load * load * length * length / 24.0).cbrt();
        assert!(
            tension < cable,
            "w = {load}: tension {tension} is not below the cable value {cable}"
        );
    }
}

#[test]
fn limp_strip_approaches_the_cable_tension() {
    let load = 10.0;
    let length = 40.0;
    let (beam, axial) = steel_strip(load, 1.0e-4);
    let restrained = restrained_ends(&beam, axial).unwrap();
    let cable = (axial * load * load * length * length / 24.0).cbrt();
    assert_close(restrained.tension, cable, 0.02, 0.0);
}

fn pinned_tension(kl: f64) -> BeamSolution {
    let k = kl / L;
    uniform_simple_beam(L, EI, W, Kernel::AxialTension { force: k * k * EI })
        .solve()
        .unwrap()
}

#[test]
fn tension_midspan_matches_the_membrane_closed_form() {
    for kl in [10.0, 30.0, 36.0, 100.0, 800.0] {
        let k = kl / L;
        let solution = pinned_tension(kl);
        let sech = 1.0 / (kl / 2.0).cosh();
        assert_close(
            solution.moment(L / 2.0),
            W / (k * k) * (1.0 - sech),
            1e-10,
            0.0,
        );
        let force = k * k * EI;
        assert_close(
            solution.deflection(L / 2.0),
            -(W / force) * (L * L / 8.0 - (1.0 - sech) / (k * k)),
            1e-10,
            0.0,
        );
        let y_peak = solution.max_deflection().abs();
        let m_peak = solution.max_moment().abs();
        assert!(
            solution.deflection(L).abs() <= 1e-10 * y_peak,
            "kL = {kl}: y(L) = {:e}, peak {y_peak:e}",
            solution.deflection(L)
        );
        assert!(
            solution.moment(L).abs() <= 1e-10 * m_peak,
            "kL = {kl}: M(L) = {:e}, peak {m_peak:e}",
            solution.moment(L)
        );
    }
}

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

#[test]
fn stiff_tension_keeps_boundaries_equilibrium_and_mirror_symmetry() {
    for kl in [50.0, 400.0] {
        let k = kl / L;
        let loads = vec![
            Load::PointForce {
                magnitude: 3.0,
                at: 0.27 * L,
            },
            Load::PointMoment {
                magnitude: 5.0,
                at: 0.61 * L,
            },
        ];
        let mut beam = Beam::new(L, EI, EndCondition::Fixed, EndCondition::Pinned, loads);
        beam.kernel = Kernel::AxialTension { force: k * k * EI };
        let solution = beam.solve().unwrap();
        let (left, right) = (solution.state_at(0.0), solution.state_at(L));
        let peaks = component_peaks(&solution);
        for (end, state) in [(beam.left, &left), (beam.right, &right)] {
            for residual in end_residuals(end, state) {
                let scale = peaks.into_iter().fold(0.0, f64::max).max(1.0);
                assert!(
                    residual.abs() <= 1e-10 * scale,
                    "kL = {kl}: residual {residual:e}"
                );
            }
        }
        let reactions = solution.reactions();
        assert_close(
            reactions.ra.unwrap() + reactions.rb.unwrap(),
            sum_forces(&beam.loads),
            1e-10,
            0.0,
        );
        let mut mirrored = Beam::new(
            L,
            EI,
            beam.right,
            beam.left,
            vec![mirror_load(beam.loads[0], L), mirror_load(beam.loads[1], L)],
        );
        mirrored.kernel = beam.kernel;
        let mirror = mirrored.solve().unwrap();
        for x in [0.4, 1.3, 2.9, 4.5, 5.5, 7.7, 9.1, 9.6] {
            let reflected = mirror.state_at(L - x);
            let expected = State::new(reflected[0], -reflected[1], reflected[2], -reflected[3]);
            let actual = solution.state_at(x);
            for c in 0..4 {
                if peaks[c] == 0.0 {
                    continue;
                }
                let difference = (actual[c] - expected[c]).abs() / peaks[c];
                assert!(
                    difference <= 1e-11,
                    "kL = {kl} at {x}, component {c}: normalized difference {difference:e}"
                );
            }
        }
    }
}

#[test]
fn state_at_a_point_moment_is_the_right_hand_limit() {
    let magnitude = 5.0;
    let at = 0.61 * L;
    for kl in [50.0, 400.0] {
        let k = kl / L;
        let mut beam = Beam::new(
            L,
            EI,
            EndCondition::Fixed,
            EndCondition::Pinned,
            vec![
                Load::PointForce {
                    magnitude: 3.0,
                    at: 0.27 * L,
                },
                Load::PointMoment { magnitude, at },
            ],
        );
        beam.kernel = Kernel::AxialTension { force: k * k * EI };
        let solution = beam.solve().unwrap();
        let jump = solution.moment(at) - solution.moment(at - 1e-8);
        assert_close(jump, -magnitude, 1e-6, 1e-8);
    }
}

#[test]
fn tension_beyond_the_segment_cap_is_unsupported() {
    let kl = 1.0e6;
    let k = kl / L;
    let result = uniform_simple_beam(L, EI, W, Kernel::AxialTension { force: k * k * EI }).solve();
    assert!(
        matches!(result, Err(BeamError::UnsupportedCombination(_))),
        "got {result:?}"
    );
}

#[test]
fn compression_brackets_each_euler_load() {
    use EndCondition::{Fixed, Free, Pinned};
    let euler = euler_load(EI, L);
    let root: f64 = 4.493_409_457_909_064;
    let cases = [
        (Pinned, Pinned, euler),
        (Fixed, Free, euler / 4.0),
        (Pinned, Fixed, root * root * EI / (L * L)),
        (Fixed, Fixed, 4.0 * euler),
    ];
    for (left, right, critical) in cases {
        let solve = |factor: f64| {
            let mut beam = Beam::new(L, EI, left, right, vec![Load::uniform(W, 0.0, L)]);
            beam.kernel = Kernel::AxialCompression {
                force: factor * critical,
            };
            beam.solve()
        };
        assert!(
            solve(0.99).is_ok(),
            "{left:?}-{right:?} just below the Euler load should solve"
        );
        assert!(
            matches!(solve(1.01), Err(BeamError::AtOrAboveCriticalLoad { .. })),
            "{left:?}-{right:?} just above the Euler load should be rejected"
        );
    }
}
