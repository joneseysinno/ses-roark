//! Phase 2: Roark 8.2 composite beams, 8.6 elastic supports, 8.11 wide beams.

use roark::beams::composite::{
    BimetalLayer, Layer, bimetal_curvature, flexural_rigidity, layer_stress, neutral_axis,
};
use roark::beams::elastic_supports::wall_rotational_stiffness;
use roark::beams::wide_beams::{
    anticlastic_curvature, flexural_rigidity as plate_rigidity, peak_stress,
};
use roark::beams::{Beam, EndCondition};

use crate::support::{assert_close, mixed_loads};

const L: f64 = 10.0;
const EI: f64 = 1.0e4;

#[test]
fn infinitely_stiff_springs_match_fixed() {
    let rigid = EndCondition::Elastic {
        vertical: f64::INFINITY,
        rotational: f64::INFINITY,
    };
    let elastic = Beam::new(L, EI, rigid, EndCondition::Pinned, mixed_loads(L))
        .solve()
        .unwrap();
    let fixed = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Pinned,
        mixed_loads(L),
    )
    .solve()
    .unwrap();
    for x in [0.0, 2.5, 4.0, 7.5, L] {
        assert_close(elastic.moment(x), fixed.moment(x), 1e-9, 1e-9);
        assert_close(elastic.deflection(x), fixed.deflection(x), 1e-9, 1e-12);
    }
}

#[test]
fn very_stiff_springs_approach_fixed() {
    let stiff = EndCondition::Elastic {
        vertical: 1e14,
        rotational: 1e14,
    };
    let elastic = Beam::new(L, EI, EndCondition::Pinned, stiff, mixed_loads(L))
        .solve()
        .unwrap();
    let fixed = Beam::new(
        L,
        EI,
        EndCondition::Pinned,
        EndCondition::Fixed,
        mixed_loads(L),
    )
    .solve()
    .unwrap();
    for x in [0.0, 3.0, 6.0, L] {
        assert_close(elastic.moment(x), fixed.moment(x), 1e-6, 1e-6);
    }
}

#[test]
fn released_rotational_spring_matches_pinned() {
    let hinge = EndCondition::Elastic {
        vertical: f64::INFINITY,
        rotational: 0.0,
    };
    let elastic = Beam::new(L, EI, EndCondition::Fixed, hinge, mixed_loads(L))
        .solve()
        .unwrap();
    let pinned = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Pinned,
        mixed_loads(L),
    )
    .solve()
    .unwrap();
    for x in [0.0, 5.0, L] {
        assert_close(elastic.deflection(x), pinned.deflection(x), 1e-9, 1e-12);
    }
}

#[test]
fn released_springs_match_free() {
    let released = EndCondition::Elastic {
        vertical: 0.0,
        rotational: 0.0,
    };
    let elastic = Beam::new(L, EI, EndCondition::Fixed, released, mixed_loads(L))
        .solve()
        .unwrap();
    let free = Beam::new(
        L,
        EI,
        EndCondition::Fixed,
        EndCondition::Free,
        mixed_loads(L),
    )
    .solve()
    .unwrap();
    assert_close(elastic.deflection(L), free.deflection(L), 1e-9, 1e-12);
    assert_eq!(elastic.reactions().rb, None);
}

#[test]
fn spring_reaction_is_proportional_to_deflection() {
    let spring = EndCondition::Elastic {
        vertical: 500.0,
        rotational: 0.0,
    };
    let solution = Beam::new(L, EI, EndCondition::Fixed, spring, mixed_loads(L))
        .solve()
        .unwrap();
    assert_close(
        solution.reactions().rb.unwrap(),
        -500.0 * solution.deflection(L),
        1e-9,
        1e-12,
    );
    assert_close(solution.moment(L), 0.0, 0.0, 1e-9);
}

#[test]
fn identical_layers_form_a_homogeneous_section() {
    let layer = Layer {
        modulus: 2.0e5,
        width: 3.0,
        thickness: 0.5,
    };
    let layers = [layer, layer];
    assert_close(neutral_axis(&layers), 0.5, 1e-12, 0.0);
    let rigidity = 2.0e5 * 3.0 * 1.0_f64.powi(3) / 12.0;
    assert_close(flexural_rigidity(&layers), rigidity, 1e-12, 0.0);
    let moment = 40.0;
    assert_close(
        layer_stress(&layers, moment, 1.0),
        -moment * 0.5 / (3.0 / 12.0),
        1e-12,
        0.0,
    );
}

#[test]
fn transformed_section_with_stiffer_bottom_layer() {
    let layers = [
        Layer {
            modulus: 3.0,
            width: 1.0,
            thickness: 1.0,
        },
        Layer {
            modulus: 1.0,
            width: 1.0,
            thickness: 1.0,
        },
    ];
    let axis = (3.0 * 0.5 + 1.0 * 1.5) / 4.0;
    assert_close(neutral_axis(&layers), axis, 1e-12, 0.0);
    let rigidity = 3.0 * (1.0 / 12.0 + (0.5 - axis).powi(2)) + (1.0 / 12.0 + (1.5 - axis).powi(2));
    assert_close(flexural_rigidity(&layers), rigidity, 1e-12, 0.0);
}

#[test]
fn equal_bimetal_layers_follow_timoshenko() {
    let bottom = BimetalLayer {
        modulus: 1.0e5,
        thickness: 0.5,
        expansion: 1.0e-5,
    };
    let top = BimetalLayer {
        expansion: 2.0e-5,
        ..bottom
    };
    let curvature = bimetal_curvature(bottom, top, 100.0);
    assert_close(curvature, -1.5 * 1.0e-5 * 100.0 / 1.0, 1e-12, 0.0);
}

#[test]
fn wall_stiffness_and_wide_beam_helpers() {
    assert_close(
        wall_rotational_stiffness(2.0e5, 0.3, 0.1),
        std::f64::consts::PI * 2.0e5 * 0.1 * 0.09 / 16.67,
        1e-12,
        0.0,
    );
    assert_close(plate_rigidity(2.0e5, 1.0, 0.3), 2.0e5 / 0.91, 1e-12, 0.0);
    assert_close(anticlastic_curvature(0.01, 0.3), -0.003, 1e-12, 0.0);
    assert_close(
        peak_stress(12.0, 4.0, 0.5),
        6.0 * 12.0 / (4.0 * 0.25),
        1e-12,
        0.0,
    );
}
