use roark::beams::{Beam, EndCondition, Kernel, Load};

/// Simply supported beam under full-span uniform load `w` with the given kernel.
pub fn uniform_simple_beam(length: f64, ei: f64, w: f64, kernel: Kernel) -> Beam {
    let mut beam = Beam::new(
        length,
        ei,
        EndCondition::Pinned,
        EndCondition::Pinned,
        vec![Load::uniform(w, 0.0, length)],
    );
    beam.kernel = kernel;
    beam
}
