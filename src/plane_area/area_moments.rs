use super::Polygon;

/// Signed `(A, ∫x dA, ∫y dA)` by the shoelace formulas.
pub(crate) fn area_moments(polygons: &[Polygon]) -> (f64, f64, f64) {
    let (mut area, mut sx, mut sy) = (0.0, 0.0, 0.0);
    for polygon in polygons {
        let v = &polygon.vertices;
        for i in 0..v.len() {
            let (x0, y0) = v[i];
            let (x1, y1) = v[(i + 1) % v.len()];
            let cross = x0 * y1 - x1 * y0;
            area += cross / 2.0;
            sx += (x0 + x1) * cross / 6.0;
            sy += (y0 + y1) * cross / 6.0;
        }
    }
    (area, sx, sy)
}
