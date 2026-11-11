use super::{PrincipalAxes, SectionProperties};

impl SectionProperties {
    /// Principal centroidal second moments and the major-axis angle.
    pub fn principal_axes(&self) -> PrincipalAxes {
        let mean = 0.5 * (self.ix + self.iy);
        let radius = (0.25 * (self.ix - self.iy).powi(2) + self.ixy * self.ixy).sqrt();
        PrincipalAxes {
            i_max: mean + radius,
            i_min: mean - radius,
            angle: 0.5 * (-2.0 * self.ixy).atan2(self.ix - self.iy),
        }
    }
}
