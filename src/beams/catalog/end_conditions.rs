use super::RoarkEnd;
use crate::beams::end_condition::EndCondition;

impl RoarkEnd {
    /// `(left, right)` end conditions for this Table 8.1 row.
    pub fn end_conditions(self) -> (EndCondition, EndCondition) {
        use EndCondition::{Fixed, Free, Guided, Pinned};
        match self {
            RoarkEnd::A => (Free, Fixed),
            RoarkEnd::B => (Guided, Fixed),
            RoarkEnd::C => (Pinned, Fixed),
            RoarkEnd::D => (Fixed, Fixed),
            RoarkEnd::E => (Pinned, Pinned),
            RoarkEnd::F => (Guided, Pinned),
        }
    }
}
