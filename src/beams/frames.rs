//! Rigid frames (Roark 8.4, Table 8.2) by the 2D direct-stiffness method.
//!
//! Members are prismatic Euler-Bernoulli elements. Member loads use the
//! [`Load`] conventions in member-local coordinates (local `x` from the start
//! node, local `y` rotated +90° from it; positive force magnitudes act toward
//! local −y). Fixed-end actions come from the beam engine, so member results
//! are exact. Global `y` is up and moments are counterclockwise positive.

mod fixed_end_actions;
mod member_rotation;
mod member_solution;
mod member_stiffness;
mod solve;

use super::beam::BeamSolution;
use super::load::Load;

/// Restrained global degrees of freedom at a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Support {
    pub ux: bool,
    pub uy: bool,
    pub rz: bool,
}

impl Support {
    pub const FREE: Support = Support {
        ux: false,
        uy: false,
        rz: false,
    };
    pub const PINNED: Support = Support {
        ux: true,
        uy: true,
        rz: false,
    };
    pub const FIXED: Support = Support {
        ux: true,
        uy: true,
        rz: true,
    };
    /// Restrains vertical translation only.
    pub const ROLLER: Support = Support {
        ux: false,
        uy: true,
        rz: false,
    };
}

/// Applied nodal force components (global) and counterclockwise moment.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NodalLoad {
    pub fx: f64,
    pub fy: f64,
    pub mz: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub support: Support,
    pub load: NodalLoad,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub start: usize,
    pub end: usize,
    pub modulus: f64,
    pub area: f64,
    pub inertia: f64,
    pub loads: Vec<Load>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub nodes: Vec<Node>,
    pub members: Vec<Member>,
}

/// Nodal displacements `[ux, uy, rz]`, support reactions `[Fx, Fy, Mz]`,
/// member end forces in local coordinates acting on the member
/// `[N1, V1, M1, N2, V2, M2]`, and a transverse beam solution per member.
#[derive(Debug, Clone, PartialEq)]
pub struct FrameSolution {
    pub displacements: Vec<[f64; 3]>,
    pub reactions: Vec<[f64; 3]>,
    pub end_forces: Vec<[f64; 6]>,
    pub members: Vec<BeamSolution>,
}
