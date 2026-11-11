//! Winkler-foundation kernel (Roark 8.5, Table 8.5), `β = (k_o / 4EI)^¼`.
//!
//! With `u = βx`, the Hetényi functions are `F1 = cosh u cos u`,
//! `F2 = cosh u sin u + sinh u cos u`, `F3 = sinh u sin u`,
//! `F4 = cosh u sin u − sinh u cos u`.

pub(crate) mod curvature_response;
pub(crate) mod f_functions;
pub(crate) mod one_minus_f1;
pub(crate) mod ramp_response;
pub(crate) mod transfer;
pub(crate) mod u_minus_half_f2;
pub(crate) mod uniform_response;
