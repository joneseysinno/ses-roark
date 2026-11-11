use super::Polygon;

impl Polygon {
    /// Counterclockwise rectangle with lower-left corner `(x0, y0)`.
    pub fn rectangle(x0: f64, y0: f64, width: f64, height: f64) -> Polygon {
        Polygon {
            vertices: vec![
                (x0, y0),
                (x0 + width, y0),
                (x0 + width, y0 + height),
                (x0, y0 + height),
            ],
        }
    }
}
