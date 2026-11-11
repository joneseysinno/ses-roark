/// Table 8.4 denominators at `βl`, computed rather than tabulated:
/// `C11 = sinh²βl − sin²βl`, `C12 = cosh βl sinh βl + cos βl sin βl`,
/// `C13 = cosh βl sinh βl − cos βl sin βl`, `C14 = sinh²βl + sin²βl`.
pub fn table_8_4_denominators(beta_l: f64) -> [f64; 4] {
    let (s, c) = beta_l.sin_cos();
    let (sh, ch) = (beta_l.sinh(), beta_l.cosh());
    [
        sh * sh - s * s,
        ch * sh + c * s,
        ch * sh - c * s,
        sh * sh + s * s,
    ]
}
