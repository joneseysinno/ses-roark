//! Errors raised by beam analyses.

/// Failure modes of the Chapter 8 solvers.
#[derive(Debug, Clone, PartialEq)]
pub enum BeamError {
    /// The supports leave a rigid-body mechanism (singular boundary system).
    UnstableSupports,
    /// Axial compression at or above the elastic critical load.
    AtOrAboveCriticalLoad { force: f64, critical: f64 },
    /// Non-physical or inconsistent input.
    InvalidInput(String),
    /// A combination of options the formulation does not cover.
    UnsupportedCombination(&'static str),
    /// An iterative procedure failed to converge.
    NoConvergence(&'static str),
}

impl std::error::Error for BeamError {}
