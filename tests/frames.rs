//! Phase 8: Roark 8.4 rigid frames (Table 8.2) by direct stiffness.

use roark::beams::frames::{Frame, Member, NodalLoad, Node, Support};
use roark::beams::{Beam, EndCondition, Load};

use crate::support::{assert_close, portal};

const H: f64 = 4.0;
const SPAN: f64 = 6.0;

#[test]
fn fixed_base_portal_with_stiff_beam_sways_as_guided_columns() {
    let solution = portal(H, SPAN, Support::FIXED, 1.0e7, 1.0, 0.0)
        .solve()
        .unwrap();
    assert_close(solution.displacements[1][0], H.powi(3) / 24.0, 1e-5, 0.0);
    assert_close(solution.reactions[0][2].abs(), H / 4.0, 1e-5, 0.0);
    assert_close(
        solution.reactions[0][0] + solution.reactions[3][0],
        -1.0,
        1e-9,
        0.0,
    );
}

#[test]
fn pinned_base_portal_under_lateral_load() {
    let beam_inertia = 2.0;
    let solution = portal(H, SPAN, Support::PINNED, beam_inertia, 1.0, 0.0)
        .solve()
        .unwrap();
    let sway = H * H / 12.0 * (SPAN / beam_inertia + 2.0 * H);
    assert_close(solution.displacements[1][0], sway, 1e-5, 0.0);
    assert_close(solution.end_forces[0][5].abs(), H / 2.0, 1e-5, 0.0);
    assert_close(solution.reactions[0][2], 0.0, 0.0, 1e-12);
}

#[test]
fn pinned_base_portal_under_gravity_load() {
    let (beam_inertia, w) = (2.0, 1.0);
    let solution = portal(H, SPAN, Support::PINNED, beam_inertia, 0.0, w)
        .solve()
        .unwrap();
    let k = beam_inertia * H / SPAN;
    let corner = w * SPAN * SPAN / (4.0 * (3.0 + 2.0 * k));
    assert_close(solution.members[1].moment(0.0), -corner, 1e-5, 0.0);
    assert_close(solution.members[1].moment(SPAN), -corner, 1e-5, 0.0);
    assert_close(solution.reactions[0][0].abs(), corner / H, 1e-5, 0.0);
    assert_close(
        solution.reactions[0][1] + solution.reactions[3][1],
        w * SPAN,
        1e-9,
        0.0,
    );
}

#[test]
fn single_member_degenerates_to_table_8_1() {
    let loads = vec![
        Load::PointForce {
            magnitude: 3.0,
            at: 2.5,
        },
        Load::uniform(1.5, 1.0, 9.0),
    ];
    let frame = Frame {
        nodes: vec![
            Node {
                x: 0.0,
                y: 0.0,
                support: Support::FIXED,
                load: NodalLoad::default(),
            },
            Node {
                x: 10.0,
                y: 0.0,
                support: Support::FIXED,
                load: NodalLoad::default(),
            },
        ],
        members: vec![Member {
            start: 0,
            end: 1,
            modulus: 1.0e4,
            area: 1.0,
            inertia: 1.0,
            loads: loads.clone(),
        }],
    }
    .solve()
    .unwrap();
    let beam = Beam::new(10.0, 1.0e4, EndCondition::Fixed, EndCondition::Fixed, loads)
        .solve()
        .unwrap();
    for x in [0.0, 2.5, 6.0, 10.0] {
        assert_close(frame.members[0].moment(x), beam.moment(x), 1e-9, 1e-12);
        assert_close(
            frame.members[0].deflection(x),
            beam.deflection(x),
            1e-9,
            1e-15,
        );
    }
    let reactions = beam.reactions();
    assert_close(frame.reactions[0][1], reactions.ra.unwrap(), 1e-9, 0.0);
    assert_close(frame.reactions[1][1], reactions.rb.unwrap(), 1e-9, 0.0);
}
