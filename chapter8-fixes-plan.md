# Chapter 8 hardening — Cursor implementation plan

Scope: fix four defects found in a review of `roark::beams` and add an
independent oracle for all 36 Table 8.1 combinations. No new Roark coverage.

Baseline when this plan was written: 73 tests pass, `cargo clippy --all-targets`
is clean. Every phase must end in that state.

## Ground rules (repo conventions — do not break)

- One function per file under chapter submodules; types in their own type file;
  `foo.rs` + `foo/`, never `mod.rs`.
- `autotests = false`: a new test module only runs if it is declared in
  `tests/chapter8.rs`. New shared helpers go in `tests/support/` and are
  re-exported from `tests/support.rs`.
- Sign conventions in `lib.rs` are locked. Do not touch them.
- Do the phases in order. Phase 0 is the safety net for the refactor in Phase 1.
- Do not loosen any existing tolerance to make a test pass. If an existing test
  fails, stop and report it.
- Public API may gain items; it must not lose or rename any.

---

## Phase 0 — Oracle for all 36 Table 8.1 combinations

Today only cases 1 and 2 (12 of 36) are compared against something other than
the engine. Add three independent checks. A trial run of checks A and B against
the current code passed with worst normalized differences of 2.6e-14 and
9.6e-15, so **both must pass before any other change is made**; a failure here
is a real finding, not a test bug.

New file `tests/table_8_1_oracle.rs` (declare it in `tests/chapter8.rs`).
Use the same six loads as `every_row_and_load_case_satisfies_statics_and_boundaries`
in `tests/table_8_1.rs`, over `ENDS` = rows a–f, with `L = 10`, `EI = 1e4`.

### A. Quadrature oracle (36 combinations)

The variable-section kernel builds its transfer matrix by Gauss-Legendre
quadrature and shares no formulas with the polynomial kernel.

For each (row, load):
1. `plain = table_8_1_case(load, row, L, EI)`.
2. `variable = plain.clone()` with
   `stiffness = Stiffness::Piecewise(vec![(0.37 * L, EI), (L, EI)])`.
3. Solve both. Compare all four state components at
   `x ∈ {0, 1.3, 2.9, 4.5, 5.5, 7.7, 9.1, L}`.
4. Normalize each component's difference by that component's peak magnitude
   over 21 evenly spaced stations of the plain solution; skip a component whose
   peak is exactly zero. Require normalized difference ≤ 1e-11.

Known limit of this oracle: for cases 4, 5, 6 the variable kernel reuses the
plain kernel's (EI-free) kinematic load terms, so only the transfer matrix and
boundary solve are independent there. Checks B and C cover the rest.

### B. Mirror symmetry (36 combinations)

Reflecting the beam end-for-end must reflect the solution. This exercises the
left/right boundary-row asymmetry, which nothing else tests.

Add `tests/support/mirror_load.rs`: `mirror_load(load: Load, length: f64) -> Load`

| Load | Mirrored |
|---|---|
| `PointForce { magnitude, at }` | same magnitude, `at = L − at` |
| `PointMoment` | magnitude negated, `at = L − at` |
| `AngularDeformation` | same angle, `at = L − at` |
| `LateralDisplacement` | offset negated, `at = L − at` |
| `LinearLoad` | swap `w_start`/`w_end`; `start = L − end`, `end = L − start` |
| `ImposedCurvature` | same curvature; `start = L − end`, `end = L − start` |

For each (row, load): build the mirrored beam with
`Beam::new(L, EI, beam.right, beam.left, vec![mirror_load(..)])`, solve both,
and at `x ∈ {0.4, 1.3, 2.9, 4.5, 5.5, 7.7, 9.1, 9.6}` require
`s(x) == diag(1, −1, 1, −1) · s_mirror(L − x)` with the same normalization and
1e-11 tolerance as check A. (Stations avoid load points on purpose: `state_at`
returns the right-hand limit, which is not mirror-invariant.)

### C. Hand closed forms for cases 3–5

Add reference files in `tests/reference/` in the existing `Expected` style, or
plain assertions, for these engine-convention results (relative tolerance 1e-12):

| Beam | Load | Expected |
|---|---|---|
| Fixed–Fixed | `PointMoment { magnitude: M0, at: L/2 }` | `ma = −M0/4`, `mb = +M0/4`, `ra = +3·M0/(2L)`, `rb = −ra` |
| Pinned–Pinned | `PointMoment { magnitude: M0, at: a }` | `ra = +M0/L`, `rb = −M0/L`, `y(a) = +M0·a·(L−a)·(L−2a)/(3·EI·L)` |
| Fixed–Fixed | `AngularDeformation { angle: θ0, at: L/2 }` | `M(x) = −EI·θ0/L` at every x, `V = 0` |
| Fixed–Fixed | `LateralDisplacement { offset: Δ0, at: L/2 }` | `ra = +12·EI·Δ0/L³`, `ma = −6·EI·Δ0/L²`, `mb = +6·EI·Δ0/L²` |

These were confirmed numerically against the current engine.

**Acceptance:** 73 + new tests pass; clippy clean; no `src/` changes in this phase.

---

## Phase 1 — Segmented solve for stiff kernels (defect 1, defect 4a)

### The defect

`Beam::solve` carries the left-end state across the whole span with one transfer
matrix. For axial tension that matrix contains `sinh(kL)` and `cosh(kL)`; the two
right-end boundary rows become parallel to within `kL/sinh(kL)`, and the load
terms and homogeneous terms cancel catastrophically at evaluation.

Measured, pinned–pinned, uniform load, `Kernel::AxialTension`:

| kL | Result today |
|---|---|
| 20 | boundary residual `M(L)` ≈ 1.8e-7 (should be 0) |
| 30 | boundary residual ≈ 4.9e-4 on a midspan moment of 0.22 |
| ≥ 36 | `Err(UnstableSupports)` — a false error |

The foundation kernel is accurate up to βL = 300 but overflows `cosh` near
βL = 710 and then also reports `UnstableSupports`.

### The fix: multiple shooting

Split the span into `n` equal segments short enough that the transfer matrix of
one segment is benign, make the state at every segment start an unknown, and
solve all of them in one global linear system. This reuses `Resolved::transfer`
and `Resolved::load_terms` unchanged.

A prototype of exactly this (appendix) reproduced the closed form
`M_mid = w/k²·(1 − sech(kL/2))` and the matching midspan deflection to better
than 2e-14 relative for kL = 1, 10, 30, 36, 100, 800, and to 1e-13 at kL = 3000.

**When to segment.** Characteristic rate `c`: `k` for `Resolved::Tension`,
`beta` for `Resolved::Foundation`. Segment when `c·L > 8`, with
`n = ceil(c·L / 8)`. `Plain`, `Compression` and `Variable` are never segmented
(compression is bounded by the critical load; `Variable` is not shift-invariant).
With `n = 1` the existing 4×4 path must run unchanged, so results in the common
regime do not move.

**Cap.** The global system is dense, O(n³): about 11 ms at n = 100. If
`n > 512` return `BeamError::UnsupportedCombination("span is too long relative
to its characteristic length; model it as semi-infinite or infinite")`.

**Unknowns and equations** (4n unknowns: `s_0 … s_{n−1}`, each the state at
`x_i = i·h` *before* any action applied at `x_i`):

- 2 rows: `end_rows(left, Side::Left) · s_0 = 0`.
- For each `i < n−1`, 4 rows: `s_{i+1} − T(h)·s_i = LT_i(h)`.
- 2 rows: `end_rows(right, Side::Right) · (T(h)·s_{n−1} + LT_{n−1}(h)) = 0`.

`LT_i(h)` is the sum of `kernel.load_terms(local, h)` over the loads clipped to
segment `i` and shifted to local coordinates:

- Point actions at `a` belong to segment `i` when `x_i ≤ a < x_{i+1}`; the last
  segment also takes `a == L`. Local position `a − x_i`.
- `LinearLoad` and `ImposedCurvature`: intersect `[start, end]` with
  `[x_i, x_{i+1}]`; skip if empty; for `LinearLoad` interpolate the intensity at
  both cut points.

Equilibrate rows then columns (same idea as `solve_scaled`) before the LU, and
map a failed or non-finite solve to `UnstableSupports`.

**Evaluation.** `BeamSolution` gains the node states and the step. `state_at(x)`
picks `i = min(floor(x/h), n−1)` and returns
`T(x − x_i)·s_i + Σ load_terms(local, x − x_i)`.

One hazard the prototype exposed: deciding whether a point action is "at or
before x" must be done in **global** coordinates (`x >= at`), not after
subtracting `x_i`, or `state_at` evaluated exactly at a load point can flip
between the left and right limit depending on `n`. Decide inclusion globally,
then evaluate with local distance `max(x − at, 0)`.

**Keep working:** `BeamSolution::initial` must still be the state at `x = 0`
(it is `s_0`); `continuous`, `frames` and `reactions` read it.
`from_initial_state` keeps building unsegmented solutions.

**Also:** add the interior node positions to the points used by `extremum`
(via `breakpoints` or at the call sites) so thin boundary layers are sampled.

### Suggested files

- `src/beams/segment_count.rs` — `c·L` rule and cap.
- `src/beams/local_loads.rs` — clip and shift loads to one segment.
- `src/beams/assemble_segmented.rs` — global matrix and right-hand side.
- `src/beams/solve_segmented.rs` — equilibrated dense LU.
- Edit `solve.rs`, `state_at.rs`, `beam.rs` (new fields), `from_initial_state.rs`.

### Tests (`tests/axial.rs`, `tests/foundation.rs`)

1. Tension, pinned–pinned, uniform `w`, kL ∈ {10, 30, 36, 100, 800}:
   `M(L/2) = w/k²·(1 − 1/cosh(kL/2))` and
   `y(L/2) = −(w/P)·(L²/8 − (1 − 1/cosh(kL/2))/k²)`, relative 1e-10;
   `y(L)` and `M(L)` zero to 1e-10 of their span peaks.
2. Tension, Fixed–Pinned, kL = 50 and 400, a point force at 0.27L plus a point
   moment at 0.61L: boundary residuals, `ΣF`, and the mirror check from Phase 0.
3. `state_at` exactly at the point-moment station returns the right-hand limit
   (moment differs from the value just left of it by the applied couple) at
   kL = 50 and kL = 400.
4. Foundation βL = 800, free–free, unit force at one `1/β` from the right end:
   deflection and moment under the load equal the mirrored case (load one `1/β`
   from the left end) to 1e-9. At βL = 300 these are −0.001220 and 1.043416 for
   `EI = 1e4`, `k = 100`; βL = 800 must give the same values to 6 digits.
5. kL large enough that `n > 512`: returns `UnsupportedCombination`, never
   `UnstableSupports`.
6. All Phase 0 and pre-existing tests still pass.

**Acceptance:** all of the above; clippy clean.

---

## Phase 2 — `restrained_ends` in the membrane regime (defect 2)

### The defect

The bisection brackets the tension between 0 and the value induced by the
unstretched beam, then tries the midpoint. For a slender strip that midpoint
lands in the kL > 36 range Phase 1 fixes, so the function returned
`UnstableSupports` even though the true answer sits near kL ≈ 8.

Reproduction: steel strip, `E = 29e6`, 1 × 0.25 in section, `L = 40`,
pinned–pinned, uniform `w` (`EI = E·b·t³/12`, `EA = E·b·t`):

| w | Linear y₀/t | Today |
|---|---|---|
| 0.5 | 1.8 | T = 350.3, kL = 3.85, y/t = 0.702 |
| 1.0 | 3.5 | T = 634.2, kL = 5.18, y/t = 0.942 |
| 2.0 | 7.1 | T = 1090.4, kL = 6.80, y/t = 1.230 |
| 2.5 | 8.8 | T = 1288.9, kL = 7.39, y/t = 1.336 |
| 3, 4, 10 | 10.6, 14.1, 35.3 | `Err(UnstableSupports)` |

### The fix

Phase 1 removes the cause. Two things remain in
`src/beams/axial/restrained_ends.rs`:

1. Integrate `θ²` with the segmented solution's node positions added to the
   quadrature breakpoints, so boundary layers are resolved.
2. Any `Err` from an inner solve other than a genuine input error should be
   surfaced as `NoConvergence("restrained-end tension")`, not `UnstableSupports`.

### Tests (`tests/axial.rs`)

1. The four working rows above reproduce `T` to 1e-3 relative (regression).
2. `w = 3, 4, 10` return `Ok`. For each, check self-consistency
   `T = EA/(2L)·∫θ² dx` with an independent Simpson sum (as the existing
   `restrained_ends_induce_consistent_tension` does) to 1e-6.
3. `T` increases strictly with `w` across all seven loads.
4. Cable bound: every `T` is below `(EA·w²·L²/24)^(1/3)`.
5. Cable limit: same strip with `EI` divided by 1e4 and `w = 10`: `T` within 2%
   of the cable value.

**Acceptance:** all of the above; clippy clean.

---

## Phase 3 — Tapers that reach zero stiffness (defect 3)

### The defect

`validate` accepts `Stiffness::Tapered` with one end at zero. That is right for
the constant-strength case (existing test
`width_tapered_cantilever_has_constant_curvature`), but a depth taper
(`exponent = 3`) to a point under a tip load returns a finite tip deflection
(−0.8148 for `L = 10`, `EI = 1e4`, unit load) where the true value is unbounded.

### The rule

Near a zero-stiffness end, with ξ the distance from it, `EI ~ ξⁿ`
(`n = exponent`) and `M ~ ξᵐ`. The solution is finite and the quadrature
accurate when `M/EI` stays bounded: **`n ≤ m`**.

In `validate`, when exactly one of `ei_start`/`ei_end` is zero:

- That end must be `Free` or `Pinned`; otherwise `InvalidInput`.
- Find `m` from what acts at that end (position within the existing `tol`):
  - `PointMoment` there → `m = 0`
  - `PointForce` there, or the end is `Pinned` (reaction) → `m = 1`
  - `LinearLoad` reaching the end with nonzero intensity there → `m = 2`
  - `LinearLoad` reaching the end with zero intensity there → `m = 3`
  - Take the smallest that applies. If nothing applies (Free end, no load
    touching it), there is no restriction.
- If `exponent > m`, return `InvalidInput` with a message that names the
  exponent, the end, and the limiting load.

Put the order-of-vanishing logic in its own file (e.g.
`src/beams/tip_moment_order.rs`).

### Tests (`tests/variable_section.rs`)

1. Existing constant-strength test still passes (`n = 1`, tip force).
2. Its mirror: Fixed–Free, `ei_end = 0`, `exponent = 1`, force `w` at `L`:
   `y(L) = −w·L³/(2·EI)`, `θ(L) = −w·L²/EI`, relative 1e-10.
3. Free–Fixed, `ei_start = 0`, `exponent = 2`, uniform load 1 over the span:
   `y(0) = −L⁴/(4·EI)`, relative 1e-10.
4. Rejected with `InvalidInput`: `exponent = 3` + tip force; `exponent = 3` +
   uniform load; `exponent = 1` + tip couple; zero stiffness at a `Fixed` end.
5. Pinned zero end, `exponent = 1`, central point load, Pinned–Pinned: solves,
   and matches the same beam with `ei_start = 1e-9·EI` to 1e-6 relative at
   midspan.

Items 2 and 3 were confirmed against the current engine.

**Acceptance:** all of the above; clippy clean.

---

## Phase 4 — Small items (defect 4b, 4c)

(4a, the βL overflow, is closed by Phase 1.)

### 4b. Critical-load scan on every compression solve

`Beam::solve` calls `critical_load`, which scans 2,500 determinants up to
`25·π²EI/L²`. A compression solve costs about 24 µs against 0.2 µs for a plain
solve.

Add `src/beams/axial/first_critical_below.rs`:
`first_critical_below(beam, force) -> Result<Option<f64>, BeamError>` — the same
determinant, the same step (`0.01·π²EI/L²`), the same bisection, but scanning
only `(0, force]` and returning `None` if no sign change is found. `solve` uses
it; `critical_load` stays public and unchanged (it may be rewritten on top of
the new function as long as its results do not move).

Tests: `compression_beyond_critical_is_rejected` and
`critical_loads_match_euler` still pass; add a case just below and just above
the Euler load for each of pinned–pinned, fixed–free, fixed–pinned, fixed–fixed.

### 4c. Report where the peak is

`max_moment` and `max_deflection` return a value with no station.

- New type file `src/beams/peak.rs`: `pub struct Peak { pub x: f64, pub value: f64 }`.
- `extremum` returns `Peak` (track the station alongside the value).
- New methods `peak_moment()` and `peak_deflection()` on `BeamSolution`
  returning `Peak`. `max_moment` and `max_deflection` become wrappers returning
  `.value`, so no caller changes.

Tests: simply supported, load rising linearly from 0 at the left to `w` at the
right: `peak_moment` at `x = L/√3` with value `w·L²/(9√3)`;
`peak_deflection` at `x = 0.5193·L` (4 significant figures). Propped cantilever
(Pinned–Fixed) under uniform load: `peak_moment` is `−w·L²/8` at `x = L`.

**Acceptance:** all tests pass; clippy clean; README "Status" gains one line
noting the segmented solve and the 36-case oracle.

---

## Appendix — verified multiple-shooting prototype

This ran inside the crate as a throwaway module and produced the accuracy
quoted in Phase 1. It is a reference for the algebra, not the final layout:
split it per the one-function-per-file rule, fix the global-coordinate inclusion
described in Phase 1 (this version compares in local coordinates), add the cap,
and derive `n` from the kernel instead of taking it as an argument.

```rust
fn local_loads(loads: &[Load], x0: f64, x1: f64, last: bool) -> Vec<Load> {
    let mut out = Vec::new();
    let inside = |a: f64| a >= x0 && (a < x1 || (last && a <= x1));
    for load in loads {
        match *load {
            Load::PointForce { magnitude, at } if inside(at) => {
                out.push(Load::PointForce { magnitude, at: at - x0 })
            }
            Load::PointMoment { magnitude, at } if inside(at) => {
                out.push(Load::PointMoment { magnitude, at: at - x0 })
            }
            Load::AngularDeformation { angle, at } if inside(at) => {
                out.push(Load::AngularDeformation { angle, at: at - x0 })
            }
            Load::LateralDisplacement { offset, at } if inside(at) => {
                out.push(Load::LateralDisplacement { offset, at: at - x0 })
            }
            Load::LinearLoad { w_start, w_end, start, end } => {
                let (a, b) = (start.max(x0), end.min(x1));
                if b > a {
                    let w = |x: f64| w_start + (w_end - w_start) * (x - start) / (end - start);
                    out.push(Load::LinearLoad {
                        w_start: w(a),
                        w_end: w(b),
                        start: a - x0,
                        end: b - x0,
                    });
                }
            }
            Load::ImposedCurvature { curvature, start, end } => {
                let (a, b) = (start.max(x0), end.min(x1));
                if b > a {
                    out.push(Load::ImposedCurvature { curvature, start: a - x0, end: b - x0 });
                }
            }
            _ => {}
        }
    }
    out
}

struct Segmented {
    h: f64,
    n: usize,
    nodes: Vec<State>,
    kernel: Resolved,
    loads: Vec<Load>,
}

fn solve_segmented(beam: &Beam, n: usize) -> Option<Segmented> {
    let kernel = Resolved::from_beam(beam).ok()?;
    let h = beam.length / n as f64;
    let t = kernel.transfer(h);
    let m = 4 * n;
    let mut a = DMatrix::<f64>::zeros(m, m);
    let mut b = DVector::<f64>::zeros(m);
    for (r, row) in end_rows(beam.left, Side::Left).iter().enumerate() {
        for j in 0..4 {
            a[(r, j)] = row[j];
        }
    }
    let mut r = 2;
    for i in 0..n {
        let last = i + 1 == n;
        let lt = local_loads(&beam.loads, i as f64 * h, (i + 1) as f64 * h, last)
            .iter()
            .fold(State::zeros(), |acc, l| acc + kernel.load_terms(l, h));
        if !last {
            for p in 0..4 {
                for q in 0..4 {
                    a[(r + p, 4 * i + q)] = -t[(p, q)];
                }
                a[(r + p, 4 * (i + 1) + p)] = 1.0;
                b[r + p] = lt[p];
            }
            r += 4;
        } else {
            for row in end_rows(beam.right, Side::Right).iter() {
                let rt = row * t;
                for q in 0..4 {
                    a[(r, 4 * i + q)] = rt[q];
                }
                b[r] = -(row * lt)[(0, 0)];
                r += 1;
            }
        }
    }
    for i in 0..m {
        let s = a.row(i).amax();
        if s == 0.0 {
            return None;
        }
        a.row_mut(i).unscale_mut(s);
        b[i] /= s;
    }
    let mut column_scales = vec![0.0; m];
    for j in 0..m {
        let s = a.column(j).amax();
        if s == 0.0 {
            return None;
        }
        a.column_mut(j).unscale_mut(s);
        column_scales[j] = s;
    }
    let u = a.lu().solve(&b)?;
    let nodes = (0..n)
        .map(|i| State::from_fn(|p, _| u[4 * i + p] / column_scales[4 * i + p]))
        .collect();
    Some(Segmented { h, n, nodes, kernel, loads: beam.loads.clone() })
}

impl Segmented {
    fn state_at(&self, x: f64) -> State {
        let i = ((x / self.h).floor() as usize).min(self.n - 1);
        let x0 = i as f64 * self.h;
        local_loads(&self.loads, x0, x0 + self.h, i + 1 == self.n)
            .iter()
            .fold(self.kernel.transfer(x - x0) * self.nodes[i], |acc, l| {
                acc + self.kernel.load_terms(l, x - x0)
            })
    }
}
```

Prototype accuracy (pinned–pinned tension, uniform load, segments of `k·h ≤ 4`):

| kL | n | Midspan M, relative error | Midspan y, relative error |
|---|---|---|---|
| 1 | 1 | 1.9e-15 | 8.7e-15 |
| 30 | 8 | 1.6e-15 | 7.9e-16 |
| 36 | 9 | 9.9e-15 | 1.8e-15 |
| 100 | 25 | 6.8e-15 | 8.1e-16 |
| 800 | 200 | 1.7e-14 | 1.1e-15 |
| 3000 | 750 | 1.0e-13 | 3.7e-15 |

The plan uses `c·h ≤ 8` instead of 4 to halve `n`; expect roughly two fewer
digits than this table, still inside the 1e-10 test tolerance.
