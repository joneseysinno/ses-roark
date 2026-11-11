use crate::plane_area::{Polygon, elastic_section_modulus, plastic_modulus};

/// Shape factor `Z/S`: plastic over elastic section modulus about the horizontal axis.
pub fn shape_factor(polygons: &[Polygon]) -> f64 {
    plastic_modulus(polygons) / elastic_section_modulus(polygons)
}
