//! Variable-section kernel (Roark 8.8-8.9): Cauchy single-integral form
//! `y = y_A + θ_A·x + ∫₀ˣ (x − ξ)·M(ξ)/EI(ξ) dξ` with Gauss-Legendre
//! quadrature split at load points and `EI` breakpoints.

pub(crate) mod gauss_legendre;
pub(crate) mod integrate;
pub(crate) mod load_terms;
pub(crate) mod transfer;
