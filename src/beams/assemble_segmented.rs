use nalgebra::{DMatrix, DVector};

use super::beam::Beam;
use super::end_condition::Side;
use super::end_equations::end_rows;
use super::kernel::Resolved;
use super::local_loads::segment_load_terms;

/// Global system for the `n` segment-start states.
///
/// Unknowns are `s_0 … s_{n−1}`, each the state at `x_i = i·h` before any
/// action applied at `x_i`. Interior rows enforce
/// `s_{i+1} − T·s_i = LT_i`; the two end pairs are the beam's boundary rows.
pub(crate) fn assemble_segmented(
    kernel: &Resolved,
    beam: &Beam,
    n: usize,
) -> (DMatrix<f64>, DVector<f64>) {
    let h = beam.length / n as f64;
    let m = 4 * n;
    let mut matrix = DMatrix::<f64>::zeros(m, m);
    let mut rhs = DVector::<f64>::zeros(m);
    for (r, row) in end_rows(beam.left, Side::Left).iter().enumerate() {
        for j in 0..4 {
            matrix[(r, j)] = row[j];
        }
    }
    let mut r = 2;
    for i in 0..n {
        let x0 = i as f64 * h;
        let last = i + 1 == n;
        let x1 = if last { beam.length } else { x0 + h };
        let transfer = kernel.transfer(x1 - x0);
        let loads = segment_load_terms(kernel, &beam.loads, x0, x1, x1, last);
        if !last {
            for p in 0..4 {
                for q in 0..4 {
                    matrix[(r + p, 4 * i + q)] = -transfer[(p, q)];
                }
                matrix[(r + p, 4 * (i + 1) + p)] = 1.0;
                rhs[r + p] = loads[p];
            }
            r += 4;
        } else {
            for row in end_rows(beam.right, Side::Right).iter() {
                let carried = row * transfer;
                for q in 0..4 {
                    matrix[(r, 4 * i + q)] = carried[q];
                }
                rhs[r] = -(row * loads)[(0, 0)];
                r += 1;
            }
        }
    }
    (matrix, rhs)
}
