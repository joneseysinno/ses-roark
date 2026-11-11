use std::collections::VecDeque;

use super::ThinWalledSection;
use crate::beams::error::BeamError;

/// Flexural center `(x, y)` in the section's coordinates.
pub fn shear_center(section: &ThinWalledSection) -> Result<(f64, f64), BeamError> {
    let nodes = &section.nodes;
    let segments = &section.segments;
    if segments.is_empty() {
        return Err(BeamError::InvalidInput(
            "section has no wall segments".into(),
        ));
    }
    let mut area = 0.0;
    let (mut ax, mut ay) = (0.0, 0.0);
    for s in segments {
        let (Some(&p), Some(&q)) = (nodes.get(s.start), nodes.get(s.end)) else {
            return Err(BeamError::InvalidInput(
                "segment references a missing node".into(),
            ));
        };
        let length = (q.0 - p.0).hypot(q.1 - p.1);
        if !(length > 0.0 && s.thickness > 0.0) {
            return Err(BeamError::InvalidInput(
                "segments need positive length and thickness".into(),
            ));
        }
        let a = s.thickness * length;
        area += a;
        ax += a * (p.0 + q.0) / 2.0;
        ay += a * (p.1 + q.1) / 2.0;
    }
    let (cx, cy) = (ax / area, ay / area);

    let mut incident = vec![Vec::new(); nodes.len()];
    for (i, s) in segments.iter().enumerate() {
        incident[s.start].push(i);
        incident[s.end].push(i);
    }
    let root = segments[0].start;
    let mut parent_segment: Vec<Option<usize>> = vec![None; nodes.len()];
    let mut visited = vec![false; nodes.len()];
    let mut order = Vec::new();
    let mut queue = VecDeque::from([root]);
    visited[root] = true;
    let mut reached = 0;
    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &si in &incident[node] {
            if parent_segment[node] == Some(si) {
                continue;
            }
            let other = if segments[si].start == node {
                segments[si].end
            } else {
                segments[si].start
            };
            if visited[other] {
                return Err(BeamError::UnsupportedCombination(
                    "closed cells are not supported",
                ));
            }
            visited[other] = true;
            parent_segment[other] = Some(si);
            reached += 1;
            queue.push_back(other);
        }
    }
    if reached != segments.len() {
        return Err(BeamError::InvalidInput(
            "section walls must be connected".into(),
        ));
    }

    let pattern = |g: &dyn Fn((f64, f64)) -> f64| {
        let mut inflow = vec![0.0; nodes.len()];
        let (mut fx, mut fy, mut torque) = (0.0, 0.0, 0.0);
        for &node in order.iter().rev() {
            let Some(si) = parent_segment[node] else {
                continue;
            };
            let s = segments[si];
            let parent = if s.start == node { s.end } else { s.start };
            let (p, q) = (nodes[node], nodes[parent]);
            let length = (q.0 - p.0).hypot(q.1 - p.1);
            let (dx, dy) = ((q.0 - p.0) / length, (q.1 - p.1) / length);
            let (g0, g1) = (g(p), (g(q) - g(p)) / length);
            let entering = inflow[node];
            let total = entering * length
                + s.thickness * (g0 * length.powi(2) / 2.0 + g1 * length.powi(3) / 6.0);
            inflow[parent] += entering + s.thickness * (g0 * length + g1 * length * length / 2.0);
            fx += total * dx;
            fy += total * dy;
            torque += (p.0 * dy - p.1 * dx) * total;
        }
        (fx, fy, torque)
    };
    let (fx1, fy1, t1) = pattern(&|p| p.0 - cx);
    let (fx2, fy2, t2) = pattern(&|p| p.1 - cy);
    let det = fx1 * fy2 - fx2 * fy1;
    if det.abs() <= 1e-14 * (fx1.abs() + fy1.abs()) * (fx2.abs() + fy2.abs()) {
        return Err(BeamError::InvalidInput("degenerate section".into()));
    }
    let x = (-fx2 * t1 + fx1 * t2) / det;
    let y = -(fy2 * t1 - fy1 * t2) / det;
    Ok((x, y))
}
