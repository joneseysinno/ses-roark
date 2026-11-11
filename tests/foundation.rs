//! Phase 3: Roark 8.5, beams on elastic foundations (Tables 8.3-8.6).

use roark::beams::foundation::{
    InfiniteBeam, SemiInfiniteBeam, characteristic, table_8_3_values, table_8_4_denominators,
};
use roark::beams::state::{DEFLECTION, MOMENT, SHEAR, SLOPE};
use roark::beams::{Beam, EndCondition, Kernel, Load};

use crate::support::assert_close;

const EI: f64 = 1.0e4;
const K: f64 = 400.0;

#[test]
fn weak_foundation_converges_to_plain() {
    let loads = vec![
        Load::PointForce {
            magnitude: 2.0,
            at: 3.0,
        },
        Load::uniform(1.0, 2.0, 9.0),
    ];
    let mut weak = Beam::new(
        10.0,
        EI,
        EndCondition::Pinned,
        EndCondition::Fixed,
        loads.clone(),
    );
    weak.kernel = Kernel::ElasticFoundation { modulus: 1e-6 };
    let weak = weak.solve().unwrap();
    let plain = Beam::new(10.0, EI, EndCondition::Pinned, EndCondition::Fixed, loads)
        .solve()
        .unwrap();
    for x in [1.0, 3.0, 6.0] {
        assert_close(weak.deflection(x), plain.deflection(x), 1e-5, 0.0);
        assert_close(weak.moment(x), plain.moment(x), 1e-5, 0.0);
    }
}

#[test]
fn free_free_beam_under_full_uniform_load_settles_rigidly() {
    let mut beam = Beam::new(
        10.0,
        EI,
        EndCondition::Free,
        EndCondition::Free,
        vec![Load::uniform(2.0, 0.0, 10.0)],
    );
    beam.kernel = Kernel::ElasticFoundation { modulus: K };
    let solution = beam.solve().unwrap();
    for x in [0.0, 3.7, 10.0] {
        assert_close(solution.deflection(x), -2.0 / K, 1e-9, 0.0);
        assert_close(solution.moment(x), 0.0, 0.0, 1e-8);
    }
}

#[test]
fn infinite_beam_point_load_matches_hetenyi() {
    let beta = characteristic(EI, K);
    let beam = InfiniteBeam {
        ei: EI,
        modulus: K,
        loads: vec![Load::PointForce {
            magnitude: 5.0,
            at: 0.0,
        }],
    };
    let at_load = beam.state_at(0.0);
    assert_close(at_load[DEFLECTION], -5.0 * beta / (2.0 * K), 1e-12, 0.0);
    assert_close(at_load[MOMENT], 5.0 / (4.0 * beta), 1e-12, 0.0);
    assert_close(at_load[SHEAR], -2.5, 1e-12, 0.0);
    assert_close(beam.state_at(-1e-12)[SHEAR], 2.5, 1e-9, 0.0);
    let away = beam.state_at(2.0);
    let u = beta * 2.0;
    assert_close(
        away[DEFLECTION],
        -5.0 * beta / (2.0 * K) * (-u).exp() * (u.cos() + u.sin()),
        1e-12,
        0.0,
    );
}

#[test]
fn infinite_beam_distributed_loads_settle_locally() {
    let uniform = InfiniteBeam {
        ei: EI,
        modulus: K,
        loads: vec![Load::uniform(3.0, -60.0, 60.0)],
    };
    assert_close(uniform.state_at(0.0)[DEFLECTION], -3.0 / K, 1e-7, 0.0);
    assert_close(uniform.state_at(0.0)[MOMENT], 0.0, 0.0, 1e-7);
    let ramp = InfiniteBeam {
        ei: EI,
        modulus: K,
        loads: vec![Load::LinearLoad {
            w_start: 0.0,
            w_end: 4.0,
            start: -60.0,
            end: 60.0,
        }],
    };
    assert_close(ramp.state_at(0.0)[DEFLECTION], -2.0 / K, 1e-7, 0.0);
    assert_close(ramp.state_at(15.0)[DEFLECTION], -2.5 / K, 1e-7, 0.0);
}

#[test]
fn semi_infinite_free_end_load_matches_hetenyi() {
    let beta = characteristic(EI, K);
    let solution = SemiInfiniteBeam {
        ei: EI,
        modulus: K,
        left: EndCondition::Free,
        loads: vec![Load::PointForce {
            magnitude: 5.0,
            at: 0.0,
        }],
    }
    .solve()
    .unwrap();
    let end = solution.state_at(0.0);
    assert_close(end[DEFLECTION], -2.0 * 5.0 * beta / K, 1e-10, 0.0);
    assert_close(end[MOMENT], 0.0, 0.0, 1e-10);
    assert_close(end[SHEAR], -5.0, 1e-10, 0.0);
}

#[test]
fn semi_infinite_supported_ends_satisfy_boundaries() {
    let loads = vec![
        Load::PointForce {
            magnitude: 5.0,
            at: 2.0,
        },
        Load::uniform(1.0, 1.0, 4.0),
    ];
    let pinned = SemiInfiniteBeam {
        ei: EI,
        modulus: K,
        left: EndCondition::Pinned,
        loads: loads.clone(),
    }
    .solve()
    .unwrap()
    .state_at(0.0);
    assert_close(pinned[DEFLECTION], 0.0, 0.0, 1e-12);
    assert_close(pinned[MOMENT], 0.0, 0.0, 1e-10);
    let fixed = SemiInfiniteBeam {
        ei: EI,
        modulus: K,
        left: EndCondition::Fixed,
        loads,
    }
    .solve()
    .unwrap()
    .state_at(0.0);
    assert_close(fixed[DEFLECTION], 0.0, 0.0, 1e-12);
    assert_close(fixed[SLOPE], 0.0, 0.0, 1e-12);
}

#[test]
fn long_finite_beam_matches_semi_infinite() {
    let beta = characteristic(EI, K);
    let length = 16.0 / beta;
    let loads = vec![
        Load::PointForce {
            magnitude: 5.0,
            at: 0.0,
        },
        Load::uniform(1.0, 1.0, 3.0),
    ];
    let mut finite = Beam::new(
        length,
        EI,
        EndCondition::Free,
        EndCondition::Free,
        loads.clone(),
    );
    finite.kernel = Kernel::ElasticFoundation { modulus: K };
    let finite = finite.solve().unwrap();
    let semi = SemiInfiniteBeam {
        ei: EI,
        modulus: K,
        left: EndCondition::Free,
        loads,
    }
    .solve()
    .unwrap();
    for x in [0.0, 1.0, 2.0, 5.0] {
        assert_close(
            finite.deflection(x),
            semi.state_at(x)[DEFLECTION],
            1e-5,
            1e-12,
        );
        assert_close(finite.moment(x), semi.state_at(x)[MOMENT], 1e-5, 1e-9);
    }
}

#[test]
fn computed_tables_are_consistent() {
    assert_eq!(table_8_3_values(0.0), [1.0, 0.0, 0.0, 0.0]);
    let below = table_8_3_values(1.0 - 1e-12);
    let above = table_8_3_values(1.0 + 1e-12);
    for (a, b) in below.iter().zip(above) {
        assert_close(*a, b, 1e-10, 0.0);
    }
    let (u, h) = (0.7, 1e-5);
    let derivative =
        |i: usize| (table_8_3_values(u + h)[i] - table_8_3_values(u - h)[i]) / (2.0 * h);
    let f = table_8_3_values(u);
    assert_close(derivative(0), -f[3], 1e-8, 0.0);
    assert_close(derivative(1), 2.0 * f[0], 1e-8, 0.0);
    assert_close(derivative(2), f[1], 1e-8, 0.0);
    assert_close(derivative(3), 2.0 * f[2], 1e-8, 0.0);
    let bl = 2.3;
    let c = table_8_4_denominators(bl);
    assert_close(c[3] - c[0], 2.0 * bl.sin().powi(2), 1e-12, 0.0);
    assert_close(c[1] + c[2], 2.0 * bl.cosh() * bl.sinh(), 1e-12, 0.0);
}
