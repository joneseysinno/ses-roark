//! Axial-tension kernel (Roark 8.7, Table 8.7b): `EI·y'''' − P·y'' = −w`,
//! `k = √(P/EI)`, with `dM/dx = V + P·θ` for the vertical shear `V`.

pub(crate) mod coefficients;
pub(crate) mod curvature_response;
pub(crate) mod ramp_response;
pub(crate) mod transfer;
pub(crate) mod uniform_response;
