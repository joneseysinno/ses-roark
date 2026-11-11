/// Secondary (Vierendeel) bending at the slot ends: the shear `shear` divides
/// between the top and bottom halves in proportion to their moments of
/// inertia, and each half bends as a member fixed against rotation at both
/// slot ends, giving `M_i = V_i·L_slot/2`. Returns `(M_top, M_bottom)`.
pub fn slot_secondary_moments(
    shear: f64,
    slot_length: f64,
    inertia_top: f64,
    inertia_bottom: f64,
) -> (f64, f64) {
    let total = inertia_top + inertia_bottom;
    let share = |inertia: f64| shear * inertia / total * slot_length / 2.0;
    (share(inertia_top), share(inertia_bottom))
}
