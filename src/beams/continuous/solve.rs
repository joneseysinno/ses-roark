use nalgebra::{DMatrix, DVector};

use super::simple_span::simple_span;
use super::span_solution::span_solution;
use super::{ContinuousBeam, ContinuousSolution};
use crate::beams::end_condition::EndCondition;
use crate::beams::error::BeamError;
use crate::beams::state::SLOPE;

impl ContinuousBeam {
    /// Solve the support moments and rebuild each span.
    pub fn solve(&self) -> Result<ContinuousSolution, BeamError> {
        let n = self.spans.len();
        if n == 0 {
            return Err(BeamError::InvalidInput(
                "a continuous beam needs at least one span".into(),
            ));
        }
        if !self.settlements.is_empty() && self.settlements.len() != n + 1 {
            return Err(BeamError::InvalidInput(format!(
                "expected {} settlements, got {}",
                n + 1,
                self.settlements.len()
            )));
        }
        let end_ok = |end: EndCondition| matches!(end, EndCondition::Pinned | EndCondition::Fixed);
        if !(end_ok(self.left) && end_ok(self.right)) {
            return Err(BeamError::UnsupportedCombination(
                "continuous-beam ends must be pinned or fixed",
            ));
        }
        let delta = |j: usize| self.settlements.get(j).copied().unwrap_or(0.0);
        let simple = self
            .spans
            .iter()
            .map(simple_span)
            .collect::<Result<Vec<_>, _>>()?;
        let slope_left: Vec<f64> = simple.iter().map(|s| s.initial[SLOPE]).collect();
        let slope_right: Vec<f64> = simple
            .iter()
            .map(|s| s.state_at(s.beam.length)[SLOPE])
            .collect();
        let flexibility = |i: usize| self.spans[i].length / self.spans[i].ei;
        let chord = |i: usize| (delta(i + 1) - delta(i)) / self.spans[i].length;

        let mut matrix = DMatrix::<f64>::zeros(n + 1, n + 1);
        let mut rhs = DVector::<f64>::zeros(n + 1);
        match self.left {
            EndCondition::Fixed => {
                matrix[(0, 0)] = flexibility(0) / 3.0;
                matrix[(0, 1)] = flexibility(0) / 6.0;
                rhs[0] = slope_left[0] + chord(0);
            }
            _ => matrix[(0, 0)] = 1.0,
        }
        for j in 1..n {
            let (a, b) = (j - 1, j);
            matrix[(j, j - 1)] = flexibility(a) / 6.0;
            matrix[(j, j)] = (flexibility(a) + flexibility(b)) / 3.0;
            matrix[(j, j + 1)] = flexibility(b) / 6.0;
            rhs[j] = slope_left[b] - slope_right[a] + chord(b) - chord(a);
        }
        match self.right {
            EndCondition::Fixed => {
                matrix[(n, n - 1)] = flexibility(n - 1) / 6.0;
                matrix[(n, n)] = flexibility(n - 1) / 3.0;
                rhs[n] = -slope_right[n - 1] - chord(n - 1);
            }
            _ => matrix[(n, n)] = 1.0,
        }
        let moments = matrix.lu().solve(&rhs).ok_or(BeamError::UnstableSupports)?;
        let spans = (0..n)
            .map(|i| {
                span_solution(
                    &simple[i],
                    self.spans[i].ei,
                    (moments[i], moments[i + 1]),
                    (delta(i), delta(i + 1)),
                )
            })
            .collect();
        Ok(ContinuousSolution {
            support_moments: moments.iter().copied().collect(),
            spans,
        })
    }
}
