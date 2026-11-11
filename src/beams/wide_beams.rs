//! Beams of relatively great width (Roark 8.11).
//!
//! Lateral (anticlastic) bending is suppressed in wide beams, so they behave
//! as plate strips. Effective widths for concentrated loads on wide
//! cantilevers are empirical; callers supply them.

mod anticlastic_curvature;
mod flexural_rigidity;
mod peak_stress;

pub use anticlastic_curvature::anticlastic_curvature;
pub use flexural_rigidity::flexural_rigidity;
pub use peak_stress::peak_stress;
