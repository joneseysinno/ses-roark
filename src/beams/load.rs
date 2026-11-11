//! Applied actions on a straight beam (Roark Table 8.1 load cases 1-6).
//!
//! Force-type magnitudes are positive **downward** at the API boundary; the
//! kernels apply the sign so results follow the crate conventions (`y` up,
//! sagging `M` positive). Positions are measured from the left end.

/// Applied action on a beam span.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Load {
    /// Concentrated force (case 1). `magnitude` positive = downward.
    PointForce { magnitude: f64, at: f64 },
    /// Linearly varying distributed load from `w_start` at `start` to `w_end`
    /// at `end` (case 2). Positive = downward; uniform when equal.
    LinearLoad {
        w_start: f64,
        w_end: f64,
        start: f64,
        end: f64,
    },
    /// Concentrated couple (case 3). `magnitude` positive = counterclockwise.
    PointMoment { magnitude: f64, at: f64 },
    /// Externally created angular deformation (case 4): the slope jumps by
    /// `angle` at `at` (positive = concave-up kink).
    AngularDeformation { angle: f64, at: f64 },
    /// Externally created lateral displacement (case 5): the deflection jumps
    /// by `offset` (positive = up) at `at`.
    LateralDisplacement { offset: f64, at: f64 },
    /// Imposed free curvature over `start..end` (case 6: thermal gradient or
    /// bimetal action). Positive = concave up (`y″ > 0`).
    ImposedCurvature {
        curvature: f64,
        start: f64,
        end: f64,
    },
}
