/// Free curvature from a linear temperature gradient (Table 8.1 case 6):
/// `γ(T_bottom − T_top)/depth`; a hotter bottom gives concave-up curvature.
pub fn thermal_curvature(gamma: f64, t_top: f64, t_bottom: f64, depth: f64) -> f64 {
    gamma * (t_bottom - t_top) / depth
}
