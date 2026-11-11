# Roark Chapter 8 Beam Library — Cursor Implementation Plan

## Goal
A standalone Rust crate that reproduces Roark's Formulas for Stress and Strain,
Chapter 8 (shear/moment/slope/deflection of elastic straight beams), starting
with **Table 8.1, Case 1: simply supported beam, concentrated intermediate load**.

## Core insight — why we're not just transcribing formulas

Roark's Table 8.1 *looks* like ~40 independent formula sets (one row per
end-condition combination, one sub-case per load type), but it is actually a
single closed-form method printed out for convenience:

> For any elastic straight beam, `EI·y(x)` and `EI·θ(x)` are built from a base
> polynomial in the two unknown integration constants (`θ_A`, `y_A`) plus one
> **singularity-function (Macaulay bracket) term per applied load/reaction**.
> The four boundary unknowns (`R_A`, `M_A`, `R_B`, `M_B` as applicable) and the
> two integration constants are solved from: 2 static equilibrium equations
> (ΣF=0, ΣM=0) + 2 boundary conditions at each end (4 total, but each end
> condition only contributes the ones that are non-trivial: free/guided/
> pinned/fixed each fix a different pair of {y, θ, M, V} to zero).

If we implement *that* general method, every row of Table 8.1 is just a
different `(EndCondition, EndCondition)` pair fed into the same solver, and
every load type in Roark's tables (concentrated force, concentrated moment,
uniform load, triangular load, etc.) is just a new singularity-function
generator. Case 1 becomes the first test case that proves the engine, not a
dead-end special case we'll have to throw away later.

This also sets up a clean seam to feed real loads from `load-combinator`
into this crate later, if you want that.

## Sign & unit conventions (decide once, document in `lib.rs`, never revisit)

- **x**: left to right along the beam, origin at the left end.
- **y (deflection)**: positive **up**. A beam sagging under a downward load
  reports **negative** y. This is deliberate — confirmed with AJ.
- **M (moment)**: positive = compression in the top flange (sagging), i.e.
  the standard structural/AISC convention.
- These two choices are mutually consistent under `M = EI·y″` with no extra
  sign flip needed — sagging under this y-up convention is concave-up
  (y has a minimum at midspan, y″ > 0), which matches positive M. Good, one
  less thing to get wrong.
- **θ (slope)** = dy/dx, same sign family as y — positive θ tips the section
  upward looking in +x direction.
- **Applied loads**: enter downward point/distributed loads as a positive
  magnitude in the `Load` variants; the load-contribution functions
  internally apply the -y sign so the bracket-term math stays consistent
  with the conventions above. Document this at the `Load` enum, not just in
  a comment buried in the math.

**Trap to watch for at Milestone 6:** most published closed-form Case 1
formulas (many textbook printings, and some Roark's editions) state
deflection/slope with **y positive downward**. If those get pasted directly
into the independent reference function, every deflection and slope value
will come out sign-flipped against this crate's convention, and it'll look
like a bug in the singularity-function engine when it's actually a
convention mismatch in the reference. Negate y and θ from any textbook
source before using them as the check.

## Architecture

### 1. Singularity function primitive (`bracket.rs`)
```rust
/// Macaulay bracket <x - a>^n : 0 for x < a, (x-a)^n for x >= a.
/// n = 0 is treated as a unit step (1 at x == a, not 0^0).
pub fn bracket(x: f64, a: f64, n: i32) -> f64
```
Pure function, trivially unit-testable (step/ramp/parabola/cubic shapes).

### 2. Load model (`load.rs`)
```rust
pub enum Load {
    PointForce { magnitude: f64, at: f64 },      // + = downward
    PointMoment { magnitude: f64, at: f64 },      // + = counterclockwise (pick a convention, document it)
    UniformLoad { w: f64, start: f64, end: f64 }, // extends start..end
}
```
Each variant knows how to contribute its bracket terms to `V(x)`, `M(x)`,
`EIθ(x)`, `EIy(x)` (standard Macaulay integration of load intensity once for
shear, twice for moment, three/four times for slope/deflection). This is
where Table 8.1's "loading" sub-cases plug in — adding `TriangularLoad` or
`PartialUniformLoad` later is additive, not a rewrite.

### 3. End conditions (`boundary.rs`)
```rust
pub enum EndCondition { Free, Guided, Pinned, Fixed }
```
| Condition | Known = 0 | Unknown reaction |
|---|---|---|
| Free | M, V | none |
| Guided | V, θ | M |
| Pinned | y, M | R |
| Fixed | y, θ | R, M |

This table *is* Roark's row selector — a Table 8.1 "case number" is just a
`(left: EndCondition, right: EndCondition)` pair.

### 4. Beam + solver (`beam.rs`)
```rust
pub struct Beam {
    pub length: f64,
    pub ei: f64,
    pub left: EndCondition,
    pub right: EndCondition,
    pub loads: Vec<Load>,
}
```
`Beam::solve(&self) -> BeamSolution` assembles the linear system for the
unknown reactions/moments + integration constants (`θ_A`, `y_A`) and solves
it with `nalgebra` (`SMatrix<f64, 4, 4>` / `.lu().solve()` — the system is
at most 4x4, small enough that a dense LU is more than sufficient, no need
for anything sparse).

`BeamSolution` exposes `.shear(x)`, `.moment(x)`, `.slope(x)`,
`.deflection(x)`, `.reactions()`, `.max_moment()`.

## Milestones (small, in order)

1. **Scaffold** — `cargo new roark-beams --lib`, README with a short note
   that the crate implements the *physics* (public-domain beam theory), not
   any copied text/tables from the book.
2. **`bracket()`** — implement + unit tests for n = 0,1,2,3,4 at a few x/a
   combinations.
3. **`Load` contribution functions** — for `PointForce` only first. Test the
   raw V/M/θ/y bracket terms in isolation (no boundary solving yet).
4. **`EndCondition` table + equilibrium equations** — implement ΣF=0, ΣM=0
   as a function of `Vec<Load>`, independent of end conditions.
5. **Solver for Case 1 specifically** (`left: Pinned, right: Pinned`) — this
   is statically determinate, so it's the easiest case to get the solver
   architecture right on before generalizing to indeterminate cases (fixed
   ends, propped cantilevers) that need the full 4x4 system.
6. **Validation harness** — write a *second, independent* implementation of
   the textbook closed-form Case 1 formulas (reactions `R_A = Pb/l`,
   `R_B = Pa/l`; `M_max = Pab/l` at `x=a`; standard piecewise slope/
   deflection formulas) as a `case1_reference()` function that does **not**
   use the bracket/solver machinery at all. Assert the general engine
   matches this reference to a tight relative tolerance (e.g. `1e-9`) across
   several `(l, a, P, EI)` combinations, including edge cases `a → 0` and
   `a → l`. Cross-checking against an independently-coded reference is safer
   than hand-verifying decimal values by eye — arithmetic slips are easy to
   miss either way.
7. **Generalize the solver** to 4 unknowns and add one indeterminate case
   (e.g. fixed-fixed or propped cantilever with a point load) to prove the
   engine handles more than the determinate case. This is the real test of
   whether "one engine, many rows" actually holds.
8. **(Stretch)** Add `UniformLoad` and one more Table 8.1 row to confirm the
   load-type axis is as additive as the boundary-condition axis.

## Naming
The bare name `roark` is unclaimed on crates.io as of this check (the only
near-match is `roark-rs`, an unrelated voice-AI analytics client — no
conflict). `roark` or `roark-beams` are both available if you want to
reserve one early.

## Beyond Table 8.1 — future forms
You mentioned wanting to expand to other Roark forms beyond beams. The
`bracket` primitive is beam-specific (Macaulay's method doesn't generalize
directly to rings, plates, or curved-beam torsion in later Roark chapters —
those use different governing equations). Don't try to force those into
this same engine. Reasonable path: keep this crate scoped to Chapter 8
(straight beams) and let it prove out the "general engine over hardcoded
cases" pattern; when you get to a second Roark chapter, decide then whether
it becomes a sibling crate in a shared workspace or its own repo — no need
to design that workspace boundary now, before there's a second data point
to design it around.
