use crate::beams::stiffness::Stiffness;

/// Stiffness of a beam with a longitudinal slot from `slot_start` to
/// `slot_end`, where the combined rigidity of the two slot halves is `ei_slotted`.
pub fn slotted_stiffness(
    ei_solid: f64,
    ei_slotted: f64,
    slot_start: f64,
    slot_end: f64,
    length: f64,
) -> Stiffness {
    let mut segments = Vec::new();
    if slot_start > 0.0 {
        segments.push((slot_start, ei_solid));
    }
    segments.push((slot_end, ei_slotted));
    if slot_end < length {
        segments.push((length, ei_solid));
    }
    Stiffness::Piecewise(segments)
}
