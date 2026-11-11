use super::stiffness::Stiffness;

impl Stiffness {
    /// The constant `EI` when the distribution is [`Stiffness::Uniform`].
    pub fn uniform_value(&self) -> Option<f64> {
        match self {
            Stiffness::Uniform(ei) => Some(*ei),
            _ => None,
        }
    }
}
