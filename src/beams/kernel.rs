//! Pluggable kernels for the initial-parameter method.
//!
//! A kernel supplies the transfer matrix `T(x)` and, for shift-invariant
//! (constant-coefficient) kernels, the integrated responses used to build the
//! load terms of distributed actions.

use super::stiffness::Stiffness;

pub(crate) mod axial_compression;
pub(crate) mod axial_tension;
pub(crate) mod elastic_foundation;
pub(crate) mod plain;
pub(crate) mod variable_section;

mod curvature_response;
mod load_terms;
mod ramp_response;
mod resolve;
mod shift_invariant_load_terms;
mod transfer;
mod uniform_response;

/// Governing-equation family for a [`Beam`](super::Beam).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kernel {
    /// Common case, `EI·y'''' = −w` (Table 8.1; variable `EI` per 8.8).
    Plain,
    /// Winkler foundation of modulus `k_o` (force per length per deflection),
    /// `EI·y'''' + k_o·y = −w` (Table 8.5).
    ElasticFoundation { modulus: f64 },
    /// End compression `P` held parallel to the original axis (Table 8.7a).
    AxialCompression { force: f64 },
    /// End tension `P` held parallel to the original axis (Table 8.7b).
    AxialTension { force: f64 },
}

/// Kernel with its parameters resolved against a beam.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Resolved {
    Plain {
        ei: f64,
        shear_flexibility: f64,
    },
    Foundation {
        ei: f64,
        modulus: f64,
        beta: f64,
    },
    Compression {
        ei: f64,
        force: f64,
        k: f64,
    },
    Tension {
        ei: f64,
        force: f64,
        k: f64,
    },
    Variable {
        stiffness: Stiffness,
        length: f64,
        shear_flexibility: f64,
        breakpoints: Vec<f64>,
    },
}
