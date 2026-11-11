//! Support / end conditions (Roark Table 8.1 row selector; 8.6 elastic supports).

/// End restraint at one end of the beam.
///
/// | Condition | Known = 0 | Reaction |
/// |-----------|-----------|----------|
/// | Free      | M, V      | none     |
/// | Guided    | θ, V      | M        |
/// | Pinned    | y, M      | R        |
/// | Fixed     | y, θ      | R, M     |
///
/// `Elastic` adds a translational spring (`vertical`, force per unit
/// deflection) and a rotational spring (`rotational`, moment per radian).
/// `0` releases a component; `f64::INFINITY` fully restrains it (Roark 8.6).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EndCondition {
    Free,
    Guided,
    Pinned,
    Fixed,
    Elastic { vertical: f64, rotational: f64 },
}

/// Which end of a member an end condition applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Side {
    Left,
    Right,
}
