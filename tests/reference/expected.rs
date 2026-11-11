/// Reference reactions (`None` where the end has none) and `(x, y)` deflection probes.
pub struct Expected {
    pub ra: Option<f64>,
    pub ma: Option<f64>,
    pub rb: Option<f64>,
    pub mb: Option<f64>,
    pub deflections: Vec<(f64, f64)>,
}
