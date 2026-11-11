//! Properties of a plane area (Roark Appendix A).
//!
//! Sections are sets of simple polygons: counterclockwise polygons add area,
//! clockwise polygons (holes) subtract it. Coordinates are `(x, y)` with `y`
//! up; bending about the horizontal axis puts compression at `+y` for a
//! sagging moment.

mod area_moments;
mod clip_below;
mod elastic_section_modulus;
mod plastic_modulus;
mod principal_axes;
mod rectangle;
mod section_properties;

pub use elastic_section_modulus::elastic_section_modulus;
pub use plastic_modulus::plastic_modulus;
pub use section_properties::section_properties;

/// Simple polygon given by its vertices in order.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<(f64, f64)>,
}

/// Area, centroid, and second moments about centroidal axes parallel to `x`, `y`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionProperties {
    pub area: f64,
    pub centroid: (f64, f64),
    pub ix: f64,
    pub iy: f64,
    pub ixy: f64,
}

/// Principal second moments and the angle (radians, counterclockwise from
/// `x`) of the major principal axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrincipalAxes {
    pub i_max: f64,
    pub i_min: f64,
    pub angle: f64,
}
