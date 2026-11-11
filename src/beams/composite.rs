//! Composite beams and bimetallic strips (Roark 8.2).
//!
//! Layers are listed bottom to top; heights `z` are measured from the bottom
//! face. The transformed section uses each layer's own modulus.

mod bimetal_curvature;
mod flexural_rigidity;
mod layer_stress;
mod neutral_axis;
mod thermal_curvature;

pub use bimetal_curvature::bimetal_curvature;
pub use flexural_rigidity::flexural_rigidity;
pub use layer_stress::layer_stress;
pub use neutral_axis::neutral_axis;
pub use thermal_curvature::thermal_curvature;

/// Rectangular layer of a composite section.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Layer {
    pub modulus: f64,
    pub width: f64,
    pub thickness: f64,
}

/// One layer of a bimetallic strip (unit width).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BimetalLayer {
    pub modulus: f64,
    pub thickness: f64,
    /// Coefficient of thermal expansion.
    pub expansion: f64,
}
