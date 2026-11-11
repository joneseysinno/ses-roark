use super::AxialSense;

/// Table 8.8 functions at `kx`, computed rather than tabulated.
/// Compression: `[cos kx, sin kx, 1 − cos kx, kx − sin kx]`;
/// tension: `[cosh kx, sinh kx, cosh kx − 1, sinh kx − kx]`.
pub fn table_8_8_values(kx: f64, sense: AxialSense) -> [f64; 4] {
    match sense {
        AxialSense::Compression => {
            let (s, c) = kx.sin_cos();
            [c, s, 1.0 - c, kx - s]
        }
        AxialSense::Tension => {
            let (sh, ch) = (kx.sinh(), kx.cosh());
            [ch, sh, ch - 1.0, sh - kx]
        }
    }
}
