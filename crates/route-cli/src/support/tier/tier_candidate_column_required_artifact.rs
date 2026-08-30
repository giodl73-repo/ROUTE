//! Helper `tier_candidate_column_required_artifact`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_candidate_column_required_artifact(
    row: &TierContactWitnessInputRow,
    closure: Option<&T2ClosureDisposition>,
) -> String {
    match closure {
        Some(closure)
            if closure.disposition == "candidate-review"
                && closure.bundle_status != "bundle-ready" =>
        {
            "data/t2-blocker-closure.csv".to_string()
        }
        Some(closure) if closure.disposition == "candidate-review" => {
            closure.source_artifact.clone()
        }
        _ => row.required_artifact.clone(),
    }
}
