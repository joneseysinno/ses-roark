use super::end_condition::EndCondition;

impl EndCondition {
    /// Whether this end can develop a vertical reaction force.
    pub fn provides_vertical_reaction(&self) -> bool {
        match *self {
            EndCondition::Pinned | EndCondition::Fixed => true,
            EndCondition::Free | EndCondition::Guided => false,
            EndCondition::Elastic { vertical, .. } => vertical > 0.0,
        }
    }
}
