//! Flexural (shear) center of thin-walled open sections (Roark 8.14,
//! Table 8.12 cases computed).
//!
//! A section is a tree of straight wall segments along the wall centerline.
//! Shear flows proportional to `∫t·x̃ ds` and `∫t·ỹ ds` are accumulated from
//! the free edges; combining them to give a unit vertical or horizontal
//! resultant locates the line of action of each, whose intersection is the
//! flexural center. Closed cells are not supported.

mod angle;
mod channel;
mod i_section;
mod shear_center;
mod tee;

pub use shear_center::shear_center;

/// Thin-walled open section: centerline nodes and wall segments between them.
#[derive(Debug, Clone, PartialEq)]
pub struct ThinWalledSection {
    pub nodes: Vec<(f64, f64)>,
    pub segments: Vec<WallSegment>,
}

/// Straight wall of constant `thickness` between two nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WallSegment {
    pub start: usize,
    pub end: usize,
    pub thickness: f64,
}
