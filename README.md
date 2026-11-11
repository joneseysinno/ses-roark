# roark

Rust implementations of **public-domain** elastic mechanics theory (stress, strain, beams, plates, and related topics), organized to grow toward the full topical coverage of *Roark's Formulas for Stress and Strain*.

This crate does **not** copy text or tables from the book. It encodes governing equations and closed-form methods from classical continuum/structural mechanics.

## Status

The module tree mirrors the book's chapter topics. **Chapter 8 — beams; flexure of straight bars** (9th ed., sections 8.1–8.16) is implemented in `roark::beams`:

- One initial-parameter solver with pluggable kernels covers Table 8.1 (plain), Table 8.5 (elastic foundation), Table 8.7 (axial compression / tension), and Table 8.11 (variable section, by quadrature), with free, guided, pinned, fixed, or elastic ends.
- Dedicated modules handle composite and bimetal beams, semi-infinite and infinite foundation beams, buckling and restrained ends, shear deformation, continuous beams (three-moment equation), rigid frames (direct stiffness), unsymmetric bending, flexural centers, shear lag, thin webs, and plastic collapse.
- Book tables of numbers (8.3, 8.4, 8.8, 8.10, 8.11, 8.13) are computed by functions; empirical chart data is supplied by the caller.

Validation lives in `tests/chapter8.rs`: independent closed-form references plus equilibrium, boundary, reciprocity, superposition, and limit-case checks. See [`roark-beam-formulas-plan.md`](roark-beam-formulas-plan.md) for the original design notes.

## Layout conventions

- Top-level modules map to book chapters (e.g. `beams`, `torsion`, `flat_plates`).
- Use `foo.rs` + optional `foo/` — never `mod.rs`.
- One function per file under chapter submodules; types live in dedicated type files.

## Sign conventions (beams)

Documented once in crate docs (`lib.rs`):

- `x` along the beam, origin at the left end
- `y` positive **up** (sagging deflection is negative)
- `M` positive sagging (compression in the top flange)
- Applied downward loads enter as positive magnitudes; load contribution functions apply the internal sign
