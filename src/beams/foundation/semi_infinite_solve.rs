use nalgebra::{Matrix2, Vector2};

use super::load_response::load_response;
use super::unit_responses::unit_responses;
use super::{SemiInfiniteBeam, SemiInfiniteSolution};
use crate::beams::end_condition::Side;
use crate::beams::end_equations::end_rows;
use crate::beams::error::BeamError;
use crate::beams::load::Load;
use crate::beams::state::{State, StateRow};

impl SemiInfiniteBeam {
    /// Solve the end-conditioning force and couple at `x = 0`.
    pub fn solve(&self) -> Result<SemiInfiniteSolution, BeamError> {
        let positive = |v: f64| v > 0.0 && v.is_finite();
        if !(positive(self.ei) && positive(self.modulus)) {
            return Err(BeamError::InvalidInput(format!(
                "EI and foundation modulus must be positive, got {} and {}",
                self.ei, self.modulus
            )));
        }
        for load in &self.loads {
            let ok = match *load {
                Load::PointForce { at, .. }
                | Load::PointMoment { at, .. }
                | Load::AngularDeformation { at, .. }
                | Load::LateralDisplacement { at, .. } => at >= 0.0,
                Load::LinearLoad { start, end, .. } | Load::ImposedCurvature { start, end, .. } => {
                    start >= 0.0 && start <= end
                }
            };
            if !ok {
                return Err(BeamError::InvalidInput(format!(
                    "load {load:?} lies off the beam"
                )));
            }
        }
        let applied = self.loads.iter().fold(State::zeros(), |acc, load| {
            acc + load_response(self.ei, self.modulus, load, 0.0, true)
        });
        let responses = unit_responses(self.ei, self.modulus, 0.0, false);
        let force_column: State = responses.column(3).into_owned();
        let couple_column: State = responses.column(2).into_owned();
        let rows = end_rows(self.left, Side::Left);
        let dot = |row: &StateRow, v: &State| (row * v)[(0, 0)];
        let matrix = Matrix2::new(
            dot(&rows[0], &force_column),
            dot(&rows[0], &couple_column),
            dot(&rows[1], &force_column),
            dot(&rows[1], &couple_column),
        );
        let rhs = Vector2::new(-dot(&rows[0], &applied), -dot(&rows[1], &applied));
        let conditioning = matrix
            .lu()
            .solve(&rhs)
            .filter(|v| v.iter().all(|c| c.is_finite()))
            .ok_or(BeamError::UnstableSupports)?;
        Ok(SemiInfiniteSolution {
            beam: self.clone(),
            end_force: conditioning[0],
            end_couple: conditioning[1],
        })
    }
}
