//! Plain kernel (Roark 8.1, Table 8.1): `EI·y'''' = −w`, optionally with
//! Timoshenko shear flexibility `1/(κGA)` (Roark 8.10).

pub(crate) mod curvature_response;
pub(crate) mod ramp_response;
pub(crate) mod shear_flexibility;
pub(crate) mod transfer;
pub(crate) mod uniform_response;
