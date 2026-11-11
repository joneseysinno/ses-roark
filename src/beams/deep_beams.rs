//! Beams of relatively great depth (Roark 8.10).
//!
//! Shear deformation enters a [`Beam`](super::Beam) through `shear_rigidity`
//! (`κGA = GA/F` with form factor `F`). This module supplies form factors and
//! the elasticity correction to `Mc/I` for short rectangular beams.

mod form_factor_circle;
mod form_factor_layered;
mod form_factor_thin_tube;
mod shear_rigidity;
mod uniform_load_stress_correction;

pub use form_factor_circle::form_factor_circle;
pub use form_factor_layered::form_factor_layered;
pub use form_factor_thin_tube::form_factor_thin_tube;
pub use shear_rigidity::shear_rigidity;
pub use uniform_load_stress_correction::uniform_load_stress_correction;
