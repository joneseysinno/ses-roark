use super::beam::BeamSolution;
use super::breakpoints::breakpoints;

/// Stations passed to the peak search: the usual breakpoints, plus every
/// shooting-segment boundary so a thin end layer is sampled.
pub(crate) fn evaluation_points(solution: &BeamSolution) -> Vec<f64> {
    let mut points = breakpoints(&solution.beam);
    let n = solution.nodes.len();
    for i in 1..n {
        points.push(i as f64 * solution.step);
    }
    let length = solution.beam.length;
    points.retain(|x| (0.0..=length).contains(x));
    points.sort_by(f64::total_cmp);
    points.dedup_by(|a, b| (*a - *b).abs() <= 1e-12 * length);
    points
}
