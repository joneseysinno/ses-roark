//! Ratios of one solved beam's key responses to a reference solution
//! (Table 8.10 axial coefficients, Table 8.11 taper coefficients).

/// `subject / reference` for the peak responses and each available reaction.
/// Reaction ratios are `None` where either beam lacks the reaction or the
/// reference value is zero.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResponseRatios {
    pub max_moment: f64,
    pub max_deflection: f64,
    pub left_reaction: Option<f64>,
    pub left_moment: Option<f64>,
    pub right_reaction: Option<f64>,
    pub right_moment: Option<f64>,
}
