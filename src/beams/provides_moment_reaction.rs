use super::end_condition::EndCondition;

impl EndCondition {
    /// Whether this end can develop a reaction moment.
    pub fn provides_moment_reaction(&self) -> bool {
        match *self {
            EndCondition::Guided | EndCondition::Fixed => true,
            EndCondition::Free | EndCondition::Pinned => false,
            EndCondition::Elastic { rotational, .. } => rotational > 0.0,
        }
    }
}
