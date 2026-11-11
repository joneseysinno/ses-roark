use super::beam::Beam;
use super::end_condition::EndCondition;
use super::kernel::Kernel;
use super::load::Load;
use super::stiffness::Stiffness;

impl Beam {
    /// Uniform plain beam (Table 8.1) with constant `ei` and no shear deformation.
    pub fn new(
        length: f64,
        ei: f64,
        left: EndCondition,
        right: EndCondition,
        loads: Vec<Load>,
    ) -> Beam {
        Beam {
            length,
            stiffness: Stiffness::Uniform(ei),
            shear_rigidity: None,
            kernel: Kernel::Plain,
            left,
            right,
            loads,
        }
    }
}
