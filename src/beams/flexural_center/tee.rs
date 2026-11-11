use super::{ThinWalledSection, WallSegment};

impl ThinWalledSection {
    /// Tee with flange on `y = 0` centered on the origin and stem down to `y = −stem_depth`.
    pub fn tee(
        flange_width: f64,
        stem_depth: f64,
        flange_thickness: f64,
        stem_thickness: f64,
    ) -> ThinWalledSection {
        ThinWalledSection {
            nodes: vec![
                (-flange_width / 2.0, 0.0),
                (0.0, 0.0),
                (flange_width / 2.0, 0.0),
                (0.0, -stem_depth),
            ],
            segments: vec![
                WallSegment {
                    start: 0,
                    end: 1,
                    thickness: flange_thickness,
                },
                WallSegment {
                    start: 1,
                    end: 2,
                    thickness: flange_thickness,
                },
                WallSegment {
                    start: 1,
                    end: 3,
                    thickness: stem_thickness,
                },
            ],
        }
    }
}
