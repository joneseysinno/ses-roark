# roark

Rust implementations of **public-domain** elastic mechanics theory (stress, strain, beams, plates, and related topics), organized to grow toward the full topical coverage of *Roark's Formulas for Stress and Strain*.

This crate does **not** copy text or tables from the book. It encodes governing equations and closed-form methods from classical continuum/structural mechanics.

## Status

Each implemented chapter is a top-level module; further chapters will be added as new modules. Section properties of plane areas (Appendix A) live in `roark::plane_area`, and **Chapter 8 — beams; flexure of straight bars** (9th ed., sections 8.1–8.16) is implemented in `roark::beams`:

- One initial-parameter solver with pluggable kernels covers Table 8.1 (plain), Table 8.5 (elastic foundation), Table 8.7 (axial compression / tension), and Table 8.11 (variable section, by quadrature), with free, guided, pinned, fixed, or elastic ends.
- Tension and foundation spans that are long next to their characteristic length are solved by multiple shooting, and all 36 Table 8.1 end and load combinations are checked against an independent oracle.
- Dedicated modules handle composite and bimetal beams, semi-infinite and infinite foundation beams, buckling and restrained ends, shear deformation, continuous beams (three-moment equation), rigid frames (direct stiffness), unsymmetric bending, flexural centers, shear lag, thin webs, and plastic collapse.
- Book tables of numbers (8.3, 8.4, 8.8, 8.10, 8.11, 8.13) are computed by functions; empirical chart data is supplied by the caller.

Validation lives in `tests/chapter8.rs`: independent closed-form references plus equilibrium, boundary, reciprocity, superposition, and limit-case checks. See [`roark-beam-formulas-plan.md`](roark-beam-formulas-plan.md) for the original design notes.

## Layout conventions

- Top-level modules map to book chapters (e.g. `beams`, `plane_area`).
- Use `foo.rs` + optional `foo/` — never `mod.rs`.
- One function per file under chapter submodules; types live in dedicated type files.

## Sign conventions (beams)

Documented once in crate docs (`lib.rs`):

- `x` along the beam, origin at the left end
- `y` positive **up** (sagging deflection is negative)
- `M` positive sagging (compression in the top flange)
- Applied downward loads enter as positive magnitudes; load contribution functions apply the internal sign

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
