use super::{ThinWalledSection, WallSegment};

impl ThinWalledSection {
    /// Angle with legs along `+x` and `+y` meeting at the origin.
    pub fn angle(leg_x: f64, leg_y: f64, thickness: f64) -> ThinWalledSection {
        ThinWalledSection {
            nodes: vec![(leg_x, 0.0), (0.0, 0.0), (0.0, leg_y)],
            segments: vec![
                WallSegment {
                    start: 0,
                    end: 1,
                    thickness,
                },
                WallSegment {
                    start: 1,
                    end: 2,
                    thickness,
                },
            ],
        }
    }
}
