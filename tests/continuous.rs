//! Phase 7: Roark 8.3, continuous beams by the three-moment equation.

use roark::beams::continuous::{ContinuousBeam, Span};
use roark::beams::frames::{Frame, Member, NodalLoad, Node, Support};
use roark::beams::{Beam, EndCondition, Load};

use crate::support::assert_close;

const EI: f64 = 1.0e4;

#[test]
fn two_equal_spans_under_uniform_load() {
    let (l, w) = (6.0, 2.0);
    let span = Span {
        length: l,
        ei: EI,
        loads: vec![Load::uniform(w, 0.0, l)],
    };
    let solution = ContinuousBeam {
        spans: vec![span.clone(), span],
        left: EndCondition::Pinned,
        right: EndCondition::Pinned,
        settlements: Vec::new(),
    }
    .solve()
    .unwrap();
    assert_close(solution.support_moments[1], -w * l * l / 8.0, 1e-12, 0.0);
    let reactions = solution.support_reactions();
    assert_close(reactions[0], 3.0 * w * l / 8.0, 1e-12, 0.0);
    assert_close(reactions[1], 10.0 * w * l / 8.0, 1e-12, 0.0);
    assert_close(reactions[2], 3.0 * w * l / 8.0, 1e-12, 0.0);
    assert_close(solution.spans[0].deflection(l), 0.0, 0.0, 1e-14);
    assert_close(
        solution.spans[0].slope(l),
        solution.spans[1].slope(0.0),
        1e-12,
        1e-15,
    );
}

#[test]
fn settlement_of_the_middle_support() {
    let (l, delta) = (6.0, 0.01);
    let span = Span {
        length: l,
        ei: EI,
        loads: Vec::new(),
    };
    let solution = ContinuousBeam {
        spans: vec![span.clone(), span],
        left: EndCondition::Pinned,
        right: EndCondition::Pinned,
        settlements: vec![0.0, -delta, 0.0],
    }
    .solve()
    .unwrap();
    assert_close(
        solution.support_moments[1],
        3.0 * EI * delta / (l * l),
        1e-12,
        0.0,
    );
    assert_close(solution.spans[1].deflection(0.0), -delta, 1e-12, 0.0);
    assert_close(solution.spans[0].deflection(l), -delta, 1e-12, 0.0);
}

#[test]
fn fixed_single_span_matches_fixed_fixed_beam() {
    let loads = vec![
        Load::PointForce {
            magnitude: 4.0,
            at: 2.0,
        },
        Load::uniform(1.0, 1.0, 7.0),
    ];
    let continuous = ContinuousBeam {
        spans: vec![Span {
            length: 8.0,
            ei: EI,
            loads: loads.clone(),
        }],
        left: EndCondition::Fixed,
        right: EndCondition::Fixed,
        settlements: Vec::new(),
    }
    .solve()
    .unwrap();
    let beam = Beam::new(8.0, EI, EndCondition::Fixed, EndCondition::Fixed, loads)
        .solve()
        .unwrap();
    for x in [0.0, 2.0, 5.0, 8.0] {
        assert_close(continuous.spans[0].moment(x), beam.moment(x), 1e-10, 1e-12);
        assert_close(
            continuous.spans[0].deflection(x),
            beam.deflection(x),
            1e-10,
            1e-15,
        );
    }
}

#[test]
fn three_spans_match_the_frame_solver() {
    let spans = vec![
        Span {
            length: 5.0,
            ei: EI,
            loads: vec![Load::uniform(2.0, 0.0, 5.0)],
        },
        Span {
            length: 7.0,
            ei: 2.0 * EI,
            loads: vec![Load::PointForce {
                magnitude: 6.0,
                at: 3.0,
            }],
        },
        Span {
            length: 4.0,
            ei: EI,
            loads: vec![Load::uniform(1.0, 1.0, 4.0)],
        },
    ];
    let continuous = ContinuousBeam {
        spans: spans.clone(),
        left: EndCondition::Fixed,
        right: EndCondition::Pinned,
        settlements: Vec::new(),
    }
    .solve()
    .unwrap();
    let xs = [0.0, 5.0, 12.0, 16.0];
    let supports = [
        Support::FIXED,
        Support::ROLLER,
        Support::ROLLER,
        Support::PINNED,
    ];
    let frame = Frame {
        nodes: xs
            .iter()
            .zip(supports)
            .map(|(&x, support)| Node {
                x,
                y: 0.0,
                support,
                load: NodalLoad::default(),
            })
            .collect(),
        members: spans
            .iter()
            .enumerate()
            .map(|(i, span)| Member {
                start: i,
                end: i + 1,
                modulus: 1.0,
                area: 1.0,
                inertia: span.ei,
                loads: span.loads.clone(),
            })
            .collect(),
    }
    .solve()
    .unwrap();
    for i in 0..3 {
        assert_close(
            frame.members[i].moment(0.0),
            continuous.support_moments[i],
            1e-9,
            1e-9,
        );
        assert_close(
            frame.members[i].moment(1.7),
            continuous.spans[i].moment(1.7),
            1e-9,
            1e-9,
        );
    }
    assert_close(
        frame.members[2].moment(4.0),
        continuous.support_moments[3],
        0.0,
        1e-9,
    );
}
