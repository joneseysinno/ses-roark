use super::stiffness::Stiffness;

impl Stiffness {
    /// Flexural rigidity at station `x` of a beam of `length`.
    pub fn ei_at(&self, x: f64, length: f64) -> f64 {
        match self {
            Stiffness::Uniform(ei) => *ei,
            Stiffness::Piecewise(segments) => segments
                .iter()
                .find(|(end, _)| x <= *end)
                .or(segments.last())
                .map_or(f64::NAN, |(_, ei)| *ei),
            Stiffness::Tapered {
                ei_start,
                ei_end,
                exponent,
            } => {
                let a = ei_start.powf(1.0 / exponent);
                let b = ei_end.powf(1.0 / exponent);
                (a + (b - a) * x / length).powf(*exponent)
            }
        }
    }
}
