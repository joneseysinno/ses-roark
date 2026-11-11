use super::beam::BeamSolution;
use super::response_ratios::ResponseRatios;

pub(crate) fn ratios_between(subject: &BeamSolution, reference: &BeamSolution) -> ResponseRatios {
    let ratio = |a: Option<f64>, b: Option<f64>| match (a, b) {
        (Some(a), Some(b)) if b != 0.0 => Some(a / b),
        _ => None,
    };
    let (s, r) = (subject.reactions(), reference.reactions());
    ResponseRatios {
        max_moment: subject.max_moment() / reference.max_moment(),
        max_deflection: subject.max_deflection() / reference.max_deflection(),
        left_reaction: ratio(s.ra, r.ra),
        left_moment: ratio(s.ma, r.ma),
        right_reaction: ratio(s.rb, r.rb),
        right_moment: ratio(s.mb, r.mb),
    }
}
