use std::fmt;

use super::error::BeamError;

impl fmt::Display for BeamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeamError::UnstableSupports => {
                write!(f, "supports do not prevent a rigid-body mechanism")
            }
            BeamError::AtOrAboveCriticalLoad { force, critical } => write!(
                f,
                "axial compression {force} is at or above the critical load {critical}"
            ),
            BeamError::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            BeamError::UnsupportedCombination(msg) => write!(f, "unsupported combination: {msg}"),
            BeamError::NoConvergence(msg) => write!(f, "no convergence: {msg}"),
        }
    }
}
