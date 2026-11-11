use roark::beams::Load;

/// A point load plus a full-span uniform load.
pub fn mixed_loads(length: f64) -> Vec<Load> {
    vec![
        Load::PointForce {
            magnitude: 3.0,
            at: 0.4 * length,
        },
        Load::uniform(1.5, 0.0, length),
    ]
}
