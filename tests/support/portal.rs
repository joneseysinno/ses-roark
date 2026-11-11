use roark::beams::Load;
use roark::beams::frames::{Frame, Member, NodalLoad, Node, Support};

/// Portal frame: unit-`I` columns of `height`, a beam of `span` and
/// `beam_inertia`, `E = 1`, near-rigid axial area, lateral load at the top
/// left corner, and optional uniform gravity load on the beam.
pub fn portal(
    height: f64,
    span: f64,
    base: Support,
    beam_inertia: f64,
    lateral: f64,
    gravity: f64,
) -> Frame {
    let node = |x: f64, y: f64, support: Support, fx: f64| Node {
        x,
        y,
        support,
        load: NodalLoad {
            fx,
            fy: 0.0,
            mz: 0.0,
        },
    };
    let member = |start: usize, end: usize, inertia: f64, loads: Vec<Load>| Member {
        start,
        end,
        modulus: 1.0,
        area: 1.0e7,
        inertia,
        loads,
    };
    let beam_loads = if gravity > 0.0 {
        vec![Load::uniform(gravity, 0.0, span)]
    } else {
        Vec::new()
    };
    Frame {
        nodes: vec![
            node(0.0, 0.0, base, 0.0),
            node(0.0, height, Support::FREE, lateral),
            node(span, height, Support::FREE, 0.0),
            node(span, 0.0, base, 0.0),
        ],
        members: vec![
            member(0, 1, 1.0, Vec::new()),
            member(1, 2, beam_inertia, beam_loads),
            member(3, 2, 1.0, Vec::new()),
        ],
    }
}
