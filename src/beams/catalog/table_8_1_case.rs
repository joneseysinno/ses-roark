use super::{RoarkEnd, RoarkLoad};
use crate::beams::beam::Beam;

/// Uniform plain beam for Table 8.1 row `end`, load case `load`.
pub fn table_8_1_case(load: RoarkLoad, end: RoarkEnd, length: f64, ei: f64) -> Beam {
    let (left, right) = end.end_conditions();
    Beam::new(length, ei, left, right, vec![load.to_load(length)])
}
