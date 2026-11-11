use super::Polygon;
use super::area_moments::area_moments;
use super::clip_below::clip_below;

/// Plastic section modulus `Z = ∫|y − y_p| dA` about the horizontal
/// equal-area axis `y_p` (found by bisection).
pub fn plastic_modulus(polygons: &[Polygon]) -> f64 {
    let (area, _, sy) = area_moments(polygons);
    let below = |cut: f64| {
        polygons
            .iter()
            .map(|p| clip_below(p, cut))
            .collect::<Vec<_>>()
    };
    let ys = polygons.iter().flat_map(|p| p.vertices.iter().map(|v| v.1));
    let (mut lo, mut hi) = ys.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), y| {
        (lo.min(y), hi.max(y))
    });
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if area_moments(&below(mid)).0 < 0.5 * area {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let axis = 0.5 * (lo + hi);
    let (area_below, _, sy_below) = area_moments(&below(axis));
    let total = sy - area * axis;
    let lower = sy_below - area_below * axis;
    total - 2.0 * lower
}
