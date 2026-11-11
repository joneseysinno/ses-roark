//! Flexural rigidity `EI` along the span (Roark 8.1 uniform; 8.8-8.9 variable).

/// Flexural rigidity distribution.
#[derive(Debug, Clone, PartialEq)]
pub enum Stiffness {
    /// Constant `EI`.
    Uniform(f64),
    /// `(x_end, ei)` segments with increasing `x_end`; each covers the span up
    /// to its `x_end`, and the last one extends to the beam end.
    Piecewise(Vec<(f64, f64)>),
    /// `EI` proportional to the `exponent` power of a linearly varying section
    /// dimension: `EI(x) = [a + (b − a)·x/l]^n` with `a = ei_start^(1/n)`,
    /// `b = ei_end^(1/n)`. `exponent = 1` is a width taper and `3` a depth taper
    /// of a rectangular section (Table 8.11).
    Tapered {
        ei_start: f64,
        ei_end: f64,
        exponent: f64,
    },
}
