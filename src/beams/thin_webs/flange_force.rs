/// Additional compression in each flange from the tension field, `(V/2)·cot α`.
pub fn flange_force(shear: f64, alpha: f64) -> f64 {
    shear / (2.0 * alpha.tan())
}
