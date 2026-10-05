//! Helper `tier_candidate_column_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_candidate_column_decision(
    row: &TierContactWitnessInputRow,
    closure: Option<&T2ClosureDisposition>,
) -> &'static str {
    if row.tier.eq_ignore_ascii_case("T2")
        && closure
            .map(|closure| closure.disposition == "candidate-review")
            .unwrap_or_default()
    {
        if closure
            .map(|closure| closure.bundle_status.as_str() != "bundle-ready")
            .unwrap_or_default()
        {
            return "blocked";
        }
        return "review";
    }
    match row.witness_type.as_str() {
        "regionalizer-ready" if row.validation_status.eq_ignore_ascii_case("pass") => "selected",
        "parent-region-review" => "review",
        "tier-demotion-needed" => "demote",
        _ => "blocked",
    }
}
