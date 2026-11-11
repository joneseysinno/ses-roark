use super::RoarkLoad;
use crate::beams::composite::thermal_curvature;
use crate::beams::load::Load;

impl RoarkLoad {
    /// Engine load for a beam of `length`. Roark's clockwise `M_o` becomes the
    /// crate's counterclockwise-positive couple.
    pub fn to_load(self, length: f64) -> Load {
        match self {
            RoarkLoad::ConcentratedLoad { w, a } => Load::PointForce {
                magnitude: w,
                at: a,
            },
            RoarkLoad::DistributedLoad { wa, wl, a } => Load::LinearLoad {
                w_start: wa,
                w_end: wl,
                start: a,
                end: length,
            },
            RoarkLoad::ConcentratedMoment { mo, a } => Load::PointMoment {
                magnitude: -mo,
                at: a,
            },
            RoarkLoad::AngularDeformation { theta_o, a } => Load::AngularDeformation {
                angle: theta_o,
                at: a,
            },
            RoarkLoad::LateralDisplacement { delta_o, a } => Load::LateralDisplacement {
                offset: delta_o,
                at: a,
            },
            RoarkLoad::TemperatureDifferential {
                t1,
                t2,
                gamma,
                depth,
                a,
            } => Load::ImposedCurvature {
                curvature: thermal_curvature(gamma, t1, t2, depth),
                start: a,
                end: length,
            },
        }
    }
}
