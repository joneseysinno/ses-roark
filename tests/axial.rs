//! Phase 4: Roark 8.7, axial plus transverse loading (Tables 8.7-8.10).

use roark::beams::axial::{
    AxialSense, amplification, critical_load, restrained_ends, table_8_8_values,
};
use roark::beams::{Beam, BeamError, EndCondition, Kernel, Load};

use crate::support::{assert_close, euler_load, uniform_simple_beam};

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
