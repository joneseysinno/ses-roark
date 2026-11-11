//! Phase 10: Roark 8.15-8.16, plastic design (Table 8.13 collapse loads).

use roark::beams::continuous::{ContinuousBeam, Span};
use roark::beams::plastic::{collapse_load_factor, continuous_collapse_load_factor, shape_factor};
use roark::beams::{EndCondition, Load};
use roark::plane_area::Polygon;

use crate::support::assert_close;

const L: f64 = 10.0;
const MP: f64 = 100.0;

#[test]
fn rectangle_shape_factor_is_one_and_a_half() {
    assert_close(
        shape_factor(&[Polygon::rectangle(0.0, 0.0, 2.0, 5.0)]),
        1.5,
        1e-10,
        0.0,
    );
}

#[test]
fn uniform_load_collapse_factors() {
    let uniform = [Load::uniform(1.0, 0.0, L)];
    use EndCondition::{Fixed, Pinned};
    assert_close(
        collapse_load_factor(L, Fixed, Fixed, &uniform, MP).unwrap(),
        16.0 * MP / (L * L),
        1e-9,
        0.0,
    );
    assert_close(
        collapse_load_factor(L, Pinned, Pinned, &uniform, MP).unwrap(),
        8.0 * MP / (L * L),
        1e-9,
        0.0,
    );
    let propped = (6.0 + 4.0 * 2.0_f64.sqrt()) * MP / (L * L);
    assert_close(
        collapse_load_factor(L, Pinned, Fixed, &uniform, MP).unwrap(),
        propped,
        1e-9,
        0.0,
    );
    assert_close(
        collapse_load_factor(L, Fixed, Pinned, &uniform, MP).unwrap(),
        propped,
        1e-9,
        0.0,
    );
}

#[test]
fn point_load_collapse_factors() {
    use EndCondition::{Fixed, Free, Pinned};
    let center = [Load::PointForce {
        magnitude: 1.0,
        at: L / 2.0,
    }];
    assert_close(
        collapse_load_factor(L, Pinned, Pinned, &center, MP).unwrap(),
        4.0 * MP / L,
        1e-12,
        0.0,
    );
    assert_close(
        collapse_load_factor(L, Fixed, Fixed, &center, MP).unwrap(),
        8.0 * MP / L,
        1e-12,
        0.0,
    );
    let offset = [Load::PointForce {
        magnitude: 1.0,
        at: 3.0,
    }];
    assert_close(
        collapse_load_factor(L, Fixed, Fixed, &offset, MP).unwrap(),
        2.0 * MP * L / (3.0 * 7.0),
        1e-12,
        0.0,
    );
    let tip = [Load::PointForce {
        magnitude: 1.0,
        at: L,
    }];
    assert_close(
        collapse_load_factor(L, Fixed, Free, &tip, MP).unwrap(),
        MP / L,
        1e-12,
        0.0,
    );
}

#[test]
fn continuous_beam_collapses_in_its_weakest_span() {
    let span = Span {
        length: L,
        ei: 1.0,
        loads: vec![Load::uniform(1.0, 0.0, L)],
    };
    let beam = ContinuousBeam {
        spans: vec![span.clone(), span],
        left: EndCondition::Pinned,
        right: EndCondition::Pinned,
        settlements: Vec::new(),
    };
    let propped = (6.0 + 4.0 * 2.0_f64.sqrt()) * MP / (L * L);
    assert_close(
        continuous_collapse_load_factor(&beam, MP).unwrap(),
        propped,
        1e-9,
        0.0,
    );
}
