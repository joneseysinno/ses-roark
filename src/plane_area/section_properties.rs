use super::area_moments::area_moments;
use super::{Polygon, SectionProperties};

/// Area, centroid, and centroidal `I_x`, `I_y`, `I_xy` of a polygonal section.
pub fn section_properties(polygons: &[Polygon]) -> SectionProperties {
    let (area, sx, sy) = area_moments(polygons);
    let (cx, cy) = if area == 0.0 {
        (0.0, 0.0)
    } else {
        (sx / area, sy / area)
    };
    let (mut ixx, mut iyy, mut ixy) = (0.0, 0.0, 0.0);
    for polygon in polygons {
        let v = &polygon.vertices;
        for i in 0..v.len() {
            let (x0, y0) = v[i];
            let (x1, y1) = v[(i + 1) % v.len()];
            let cross = x0 * y1 - x1 * y0;
            ixx += (y0 * y0 + y0 * y1 + y1 * y1) * cross / 12.0;
            iyy += (x0 * x0 + x0 * x1 + x1 * x1) * cross / 12.0;
            ixy += (x0 * y1 + 2.0 * x0 * y0 + 2.0 * x1 * y1 + x1 * y0) * cross / 24.0;
        }
    }
    SectionProperties {
        area,
        centroid: (cx, cy),
        ix: ixx - area * cy * cy,
        iy: iyy - area * cx * cx,
        ixy: ixy - area * cx * cy,
    }
}
