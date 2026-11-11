use super::Polygon;
use super::section_properties::section_properties;

/// Elastic section modulus `I_x / c_max` about the horizontal centroidal axis.
pub fn elastic_section_modulus(polygons: &[Polygon]) -> f64 {
    let props = section_properties(polygons);
    let ys = polygons.iter().flat_map(|p| p.vertices.iter().map(|v| v.1));
    let (lo, hi) = ys.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), y| {
        (lo.min(y), hi.max(y))
    });
    let c = (hi - props.centroid.1).max(props.centroid.1 - lo);
    props.ix / c
}
