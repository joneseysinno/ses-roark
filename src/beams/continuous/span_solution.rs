use crate::beams::beam::BeamSolution;
use crate::beams::state::State;

/// Superpose end moments and support settlements on a simply supported span.
/// The added field is linear in moment, so its left-end state is
/// `[δ_L, θ₀, M_L, (M_R − M_L)/l]` with
/// `θ₀ = −(M_L·l/3 + M_R·l/6)/EI + (δ_R − δ_L)/l`.
pub(crate) fn span_solution(
    simple: &BeamSolution,
    ei: f64,
    moments: (f64, f64),
    settlements: (f64, f64),
) -> BeamSolution {
    let l = simple.beam.length;
    let (m_left, m_right) = moments;
    let (d_left, d_right) = settlements;
    let theta = -(m_left * l / 3.0 + m_right * l / 6.0) / ei + (d_right - d_left) / l;
    let added = State::new(d_left, theta, m_left, (m_right - m_left) / l);
    BeamSolution::from_initial_state(
        simple.beam.clone(),
        simple.kernel.clone(),
        simple.initial + added,
    )
}
