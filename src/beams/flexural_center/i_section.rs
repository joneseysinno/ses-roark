use super::{ThinWalledSection, WallSegment};

impl ThinWalledSection {
    /// I-section with web on `x = 0` (centerline depth `depth`, centered on
    /// `y = 0`) and flanges centered on the web.
    pub fn i_section(
        top_width: f64,
        bottom_width: f64,
        depth: f64,
        top_thickness: f64,
        bottom_thickness: f64,
        web_thickness: f64,
    ) -> ThinWalledSection {
        let h = depth / 2.0;
        ThinWalledSection {
            nodes: vec![
                (-top_width / 2.0, h),
                (0.0, h),
                (top_width / 2.0, h),
                (-bottom_width / 2.0, -h),
                (0.0, -h),
                (bottom_width / 2.0, -h),
            ],
            segments: vec![
                WallSegment {
                    start: 1,
                    end: 0,
                    thickness: top_thickness,
                },
                WallSegment {
                    start: 1,
                    end: 2,
                    thickness: top_thickness,
                },
                WallSegment {
                    start: 1,
                    end: 4,
                    thickness: web_thickness,
                },
                WallSegment {
                    start: 4,
                    end: 3,
                    thickness: bottom_thickness,
                },
                WallSegment {
                    start: 4,
                    end: 5,
                    thickness: bottom_thickness,
                },
            ],
        }
    }
}
