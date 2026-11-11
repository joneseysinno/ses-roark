use super::plain::shear_flexibility::shear_flexibility;
use super::{Kernel, Resolved};
use crate::beams::beam::Beam;
use crate::beams::breakpoints::breakpoints;
use crate::beams::error::BeamError;

impl Resolved {
    /// Resolve the beam's kernel; zero foundation modulus or axial force
    /// degenerates to the plain kernel, and a negative axial force flips sense.
    pub(crate) fn from_beam(beam: &Beam) -> Result<Resolved, BeamError> {
        let flexibility = shear_flexibility(beam.shear_rigidity);
        let Some(ei) = beam.stiffness.uniform_value() else {
            return match beam.kernel {
                Kernel::Plain => Ok(Resolved::Variable {
                    stiffness: beam.stiffness.clone(),
                    length: beam.length,
                    shear_flexibility: flexibility,
                    breakpoints: breakpoints(beam),
                }),
                _ => Err(BeamError::UnsupportedCombination(
                    "non-uniform stiffness requires the plain kernel",
                )),
            };
        };
        if flexibility != 0.0 && beam.kernel != Kernel::Plain {
            return Err(BeamError::UnsupportedCombination(
                "shear deformation is only supported with the plain kernel",
            ));
        }
        let plain = Resolved::Plain {
            ei,
            shear_flexibility: flexibility,
        };
        let axial = |compression: f64| {
            if compression == 0.0 {
                plain.clone()
            } else if compression > 0.0 {
                Resolved::Compression {
                    ei,
                    force: compression,
                    k: (compression / ei).sqrt(),
                }
            } else {
                Resolved::Tension {
                    ei,
                    force: -compression,
                    k: (-compression / ei).sqrt(),
                }
            }
        };
        Ok(match beam.kernel {
            Kernel::Plain => plain.clone(),
            Kernel::ElasticFoundation { modulus: 0.0 } => plain.clone(),
            Kernel::ElasticFoundation { modulus } => Resolved::Foundation {
                ei,
                modulus,
                beta: (modulus / (4.0 * ei)).powf(0.25),
            },
            Kernel::AxialCompression { force } => axial(force),
            Kernel::AxialTension { force } => axial(-force),
        })
    }
}
