use std::f64::consts::PI;
use std::sync::OnceLock;

/// 16-point Gauss-Legendre nodes and weights on `[−1, 1]`.
pub(crate) fn gauss_legendre() -> &'static ([f64; 16], [f64; 16]) {
    static RULE: OnceLock<([f64; 16], [f64; 16])> = OnceLock::new();
    RULE.get_or_init(|| {
        const N: usize = 16;
        let mut nodes = [0.0; N];
        let mut weights = [0.0; N];
        for i in 0..N {
            let mut z = (PI * (i as f64 + 0.75) / (N as f64 + 0.5)).cos();
            let mut derivative = 0.0;
            for _ in 0..100 {
                let (mut p1, mut p2) = (1.0, 0.0);
                for j in 0..N {
                    let p3 = p2;
                    p2 = p1;
                    p1 = ((2 * j + 1) as f64 * z * p2 - j as f64 * p3) / (j + 1) as f64;
                }
                derivative = N as f64 * (z * p1 - p2) / (z * z - 1.0);
                let previous = z;
                z = previous - p1 / derivative;
                if (z - previous).abs() < 1e-16 {
                    break;
                }
            }
            nodes[i] = z;
            weights[i] = 2.0 / ((1.0 - z * z) * derivative * derivative);
        }
        (nodes, weights)
    })
}
