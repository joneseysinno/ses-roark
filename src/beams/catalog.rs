//! Roark Table 8.1 case catalog (9th ed.): end-condition rows a-f and load
//! cases 1-6, mapped onto the general engine.

mod end_conditions;
mod table_8_1_case;
mod to_load;

pub use table_8_1_case::table_8_1_case;

/// Table 8.1 end-condition rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoarkEnd {
    /// (a) Left end free, right end fixed (cantilever).
    A,
    /// (b) Left end guided, right end fixed.
    B,
    /// (c) Left end simply supported, right end fixed.
    C,
    /// (d) Left end fixed, right end fixed.
    D,
    /// (e) Left end simply supported, right end simply supported.
    E,
    /// (f) Left end guided, right end simply supported.
    F,
}

/// Table 8.1 load cases, in Roark's notation and sign conventions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoarkLoad {
    /// Case 1: concentrated intermediate load `W` (downward) at `a`.
    ConcentratedLoad { w: f64, a: f64 },
    /// Case 2: distributed load varying linearly from `wa` at `a` to `wl` at `l`.
    DistributedLoad { wa: f64, wl: f64, a: f64 },
    /// Case 3: concentrated intermediate moment `M_o` at `a`, positive
    /// clockwise as drawn in the table (so `R_A = −M_o/l` for row e).
    ConcentratedMoment { mo: f64, a: f64 },
    /// Case 4: externally created angular deformation `θ_o` at `a`.
    AngularDeformation { theta_o: f64, a: f64 },
    /// Case 5: externally created lateral displacement `Δ_o` at `a`.
    LateralDisplacement { delta_o: f64, a: f64 },
    /// Case 6: temperature `t1` on top and `t2` on the bottom from `a` to `l`,
    /// expansion coefficient `gamma`, section depth `depth`.
    TemperatureDifferential {
        t1: f64,
        t2: f64,
        gamma: f64,
        depth: f64,
        a: f64,
    },
}
