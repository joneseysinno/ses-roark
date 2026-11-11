use super::beam::Beam;
use super::end_condition::EndCondition;
use super::load::Load;

/// Smallest power `m` in `M ~ ξ^m` at `station`, and the action that sets it.
///
/// `None` when that end is free of both reactions and touching loads, so the
/// taper exponent is unrestricted.
pub(crate) fn tip_moment_order(beam: &Beam, station: f64) -> Option<(u32, &'static str)> {
    let tol = 1e-12 * beam.length;
    let near = |x: f64| (x - station).abs() <= tol;
    let mut best: Option<(u32, &'static str)> = None;
    let mut consider = |order: u32, name: &'static str| {
        if best.is_none_or(|(current, _)| order < current) {
            best = Some((order, name));
        }
    };
    for load in &beam.loads {
        match *load {
            Load::PointMoment { at, .. } if near(at) => consider(0, "point moment"),
            Load::PointForce { at, .. } if near(at) => consider(1, "point force"),
            Load::LinearLoad {
                w_start,
                w_end,
                start,
                end,
            } if start <= station + tol && end >= station - tol && end >= start => {
                let intensity = if end > start {
                    let x = station.clamp(start, end);
                    w_start + (w_end - w_start) * (x - start) / (end - start)
                } else {
                    w_start
                };
                consider(if intensity == 0.0 { 3 } else { 2 }, "distributed load");
            }
            _ => {}
        }
    }
    let support = if station <= tol {
        beam.left
    } else {
        beam.right
    };
    if support == EndCondition::Pinned {
        consider(1, "pinned reaction");
    }
    best
}
