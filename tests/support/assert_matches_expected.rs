use roark::beams::BeamSolution;

use super::assert_close::assert_close;
use crate::reference::Expected;

/// Compare reactions present in `expected` and every deflection probe.
#[track_caller]
pub fn assert_matches_expected(solution: &BeamSolution, expected: &Expected) {
    let reactions = solution.reactions();
    let pairs = [
        (reactions.ra, expected.ra, "R_A"),
        (reactions.ma, expected.ma, "M_A"),
        (reactions.rb, expected.rb, "R_B"),
        (reactions.mb, expected.mb, "M_B"),
    ];
    for (actual, wanted, name) in pairs {
        match (actual, wanted) {
            (Some(a), Some(w)) => assert_close(a, w, 1e-9, 1e-9),
            (None, None) => {}
            _ => panic!("{name}: engine {actual:?} vs reference {wanted:?}"),
        }
    }
    for &(x, y) in &expected.deflections {
        assert_close(solution.deflection(x), y, 1e-9, 1e-12);
    }
}
