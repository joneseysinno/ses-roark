/// `1/(κGA)`, or zero when shear deformation is ignored.
pub(crate) fn shear_flexibility(shear_rigidity: Option<f64>) -> f64 {
    shear_rigidity.map_or(0.0, |rigidity| 1.0 / rigidity)
}
