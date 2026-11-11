//! Beams with very thin webs (Roark 8.13): elastic shear buckling of the web
//! panel and Wagner's diagonal-tension field after buckling, with tension
//! diagonals at angle `alpha` to the beam axis.

mod diagonal_tension_stress;
mod flange_force;
mod shear_buckling_coefficient;
mod shear_buckling_stress;
mod stiffener_force;

pub use diagonal_tension_stress::diagonal_tension_stress;
pub use flange_force::flange_force;
pub use shear_buckling_coefficient::shear_buckling_coefficient;
pub use shear_buckling_stress::shear_buckling_stress;
pub use stiffener_force::stiffener_force;
