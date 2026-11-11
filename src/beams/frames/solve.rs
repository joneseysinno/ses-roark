use nalgebra::{DMatrix, DVector, SVector};

use super::fixed_end_actions::fixed_end_actions;
use super::member_rotation::member_rotation;
use super::member_solution::member_solution;
use super::member_stiffness::member_stiffness;
use super::{Frame, FrameSolution};
use crate::beams::error::BeamError;

impl Frame {
    /// Assemble, solve the free degrees of freedom, and recover member results.
    pub fn solve(&self) -> Result<FrameSolution, BeamError> {
        let dofs = 3 * self.nodes.len();
        let mut stiffness = DMatrix::<f64>::zeros(dofs, dofs);
        let mut applied = DVector::<f64>::zeros(dofs);
        for (i, node) in self.nodes.iter().enumerate() {
            applied[3 * i] = node.load.fx;
            applied[3 * i + 1] = node.load.fy;
            applied[3 * i + 2] = node.load.mz;
        }
        let mut loads = applied.clone();
        let mut prepared = Vec::with_capacity(self.members.len());
        for member in &self.members {
            let (Some(a), Some(b)) = (self.nodes.get(member.start), self.nodes.get(member.end))
            else {
                return Err(BeamError::InvalidInput(format!(
                    "member {}-{} references a missing node",
                    member.start, member.end
                )));
            };
            let length = (b.x - a.x).hypot(b.y - a.y);
            let positive = |v: f64| v > 0.0 && v.is_finite();
            if !(positive(length)
                && positive(member.modulus)
                && positive(member.area)
                && positive(member.inertia))
            {
                return Err(BeamError::InvalidInput(format!(
                    "member {}-{} needs positive length, E, A, and I",
                    member.start, member.end
                )));
            }
            let local = member_stiffness(member, length);
            let rotation = member_rotation((b.x - a.x) / length, (b.y - a.y) / length);
            let fixed = fixed_end_actions(member, length)?;
            let global = rotation.transpose() * local * rotation;
            let equivalent = rotation.transpose() * fixed;
            let map = [
                3 * member.start,
                3 * member.start + 1,
                3 * member.start + 2,
                3 * member.end,
                3 * member.end + 1,
                3 * member.end + 2,
            ];
            for r in 0..6 {
                loads[map[r]] -= equivalent[r];
                for c in 0..6 {
                    stiffness[(map[r], map[c])] += global[(r, c)];
                }
            }
            prepared.push((length, local, rotation, fixed, map));
        }

        let restrained = |dof: usize| {
            let support = self.nodes[dof / 3].support;
            match dof % 3 {
                0 => support.ux,
                1 => support.uy,
                _ => support.rz,
            }
        };
        let free: Vec<usize> = (0..dofs).filter(|&d| !restrained(d)).collect();
        let mut displacement = DVector::<f64>::zeros(dofs);
        if !free.is_empty() {
            let reduced =
                DMatrix::from_fn(free.len(), free.len(), |i, j| stiffness[(free[i], free[j])]);
            let rhs = DVector::from_fn(free.len(), |i, _| loads[free[i]]);
            let lu = reduced.lu();
            let pivots = lu.u().diagonal();
            if pivots.amin() <= 1e-12 * pivots.amax() {
                return Err(BeamError::UnstableSupports);
            }
            let solved = lu.solve(&rhs).ok_or(BeamError::UnstableSupports)?;
            for (i, &d) in free.iter().enumerate() {
                displacement[d] = solved[i];
            }
        }

        let mut nodal = DVector::<f64>::zeros(dofs);
        let mut end_forces = Vec::with_capacity(self.members.len());
        let mut members = Vec::with_capacity(self.members.len());
        for (member, (length, local, rotation, fixed, map)) in self.members.iter().zip(&prepared) {
            let global_displacement = SVector::<f64, 6>::from_fn(|r, _| displacement[map[r]]);
            let local_displacement = rotation * global_displacement;
            let forces = local * local_displacement + fixed;
            let global_forces = rotation.transpose() * forces;
            for r in 0..6 {
                nodal[map[r]] += global_forces[r];
            }
            end_forces.push(std::array::from_fn(|i| forces[i]));
            members.push(member_solution(
                member,
                *length,
                &local_displacement,
                &forces,
            ));
        }
        let reactions = (0..self.nodes.len())
            .map(|n| {
                std::array::from_fn(|c| {
                    let d = 3 * n + c;
                    if restrained(d) {
                        nodal[d] - applied[d]
                    } else {
                        0.0
                    }
                })
            })
            .collect();
        let displacements = (0..self.nodes.len())
            .map(|n| std::array::from_fn(|c| displacement[3 * n + c]))
            .collect();
        Ok(FrameSolution {
            displacements,
            reactions,
            end_forces,
            members,
        })
    }
}
