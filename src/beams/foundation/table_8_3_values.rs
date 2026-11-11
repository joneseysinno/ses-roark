use crate::beams::kernel::elastic_foundation::f_functions::f_functions;

/// Table 8.3 functions `[F1, F2, F3, F4]` at `βx`, computed rather than tabulated.
pub fn table_8_3_values(beta_x: f64) -> [f64; 4] {
    f_functions(beta_x)
}
