use super::beam::Beam;
use super::load::Load;
use super::stiffness::Stiffness;

/// Sorted stations where the state may be non-smooth: ends, load positions,
/// and piecewise-stiffness boundaries.
pub(crate) fn breakpoints(beam: &Beam) -> Vec<f64> {
    let l = beam.length;
    let mut points = vec![0.0, l];
    for load in &beam.loads {
        match *load {
            Load::PointForce { at, .. }
            | Load::PointMoment { at, .. }
            | Load::AngularDeformation { at, .. }
            | Load::LateralDisplacement { at, .. } => points.push(at),
            Load::LinearLoad { start, end, .. } | Load::ImposedCurvature { start, end, .. } => {
                points.push(start);
                points.push(end);
            }
        }
    }
    if let Stiffness::Piecewise(segments) = &beam.stiffness {
        points.extend(segments.iter().map(|(x, _)| *x));
    }
    points.retain(|x| (0.0..=l).contains(x));
    points.sort_by(f64::total_cmp);
    points.dedup_by(|a, b| (*a - *b).abs() <= 1e-12 * l);
    points
}
