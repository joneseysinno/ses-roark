use roark::plane_area::Polygon;

/// Rectangle centered on the origin, rotated counterclockwise by `angle`.
pub fn rotated_rectangle(width: f64, height: f64, angle: f64) -> Polygon {
    let (s, c) = angle.sin_cos();
    let corners = [
        (-width / 2.0, -height / 2.0),
        (width / 2.0, -height / 2.0),
        (width / 2.0, height / 2.0),
        (-width / 2.0, height / 2.0),
    ];
    Polygon {
        vertices: corners
            .iter()
            .map(|&(x, y)| (x * c - y * s, x * s + y * c))
            .collect(),
    }
}
