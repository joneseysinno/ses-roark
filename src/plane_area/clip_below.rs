use super::Polygon;

/// Part of `polygon` with `y ≤ cut` (Sutherland-Hodgman against one half-plane).
pub(crate) fn clip_below(polygon: &Polygon, cut: f64) -> Polygon {
    let v = &polygon.vertices;
    let mut out = Vec::with_capacity(v.len() + 2);
    for i in 0..v.len() {
        let p = v[i];
        let q = v[(i + 1) % v.len()];
        let (p_in, q_in) = (p.1 <= cut, q.1 <= cut);
        if p_in {
            out.push(p);
        }
        if p_in != q_in {
            let t = (cut - p.1) / (q.1 - p.1);
            out.push((p.0 + t * (q.0 - p.0), cut));
        }
    }
    Polygon { vertices: out }
}
