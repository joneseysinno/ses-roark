use nalgebra::SMatrix;

/// Global-to-local transformation for a member with direction cosines `(c, s)`.
pub(crate) fn member_rotation(c: f64, s: f64) -> SMatrix<f64, 6, 6> {
    let mut t = SMatrix::<f64, 6, 6>::zeros();
    for block in [0, 3] {
        t[(block, block)] = c;
        t[(block, block + 1)] = s;
        t[(block + 1, block)] = -s;
        t[(block + 1, block + 1)] = c;
        t[(block + 2, block + 2)] = 1.0;
    }
    t
}
