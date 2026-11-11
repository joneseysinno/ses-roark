use super::{ThinWalledSection, WallSegment};

impl ThinWalledSection {
    /// Channel with web on `x = 0` (centerline depth `depth`, centered on
    /// `y = 0`) and flanges of width `flange_width` extending toward `+x`.
    pub fn channel(
        flange_width: f64,
        depth: f64,
        flange_thickness: f64,
        web_thickness: f64,
    ) -> ThinWalledSection {
        let h = depth / 2.0;
        ThinWalledSection {
            nodes: vec![(flange_width, h), (0.0, h), (0.0, -h), (flange_width, -h)],
            segments: vec![
                WallSegment {
                    start: 0,
                    end: 1,
                    thickness: flange_thickness,
                },
                WallSegment {
                    start: 1,
                    end: 2,
                    thickness: web_thickness,
                },
                WallSegment {
                    start: 2,
                    end: 3,
                    thickness: flange_thickness,
                },
            ],
        }
    }
}
