use super::load::Load;

impl Load {
    /// Uniform distributed load of intensity `w` (positive = downward) on `start..end`.
    pub fn uniform(w: f64, start: f64, end: f64) -> Load {
        Load::LinearLoad {
            w_start: w,
            w_end: w,
            start,
            end,
        }
    }
}
