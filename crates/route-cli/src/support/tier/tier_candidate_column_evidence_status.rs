//! Helper `tier_candidate_column_evidence_status`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_candidate_column_evidence_status(
    row: &TierContactWitnessInputRow,
    closure: Option<&T2ClosureDisposition>,
) -> String {
    match closure {
        Some(closure)
            if closure.disposition == "candidate-review"
                && closure.bundle_status == "bundle-ready" =>
        {
            "closure-accepted-bundle-ready".to_string()
        }
        Some(closure) if closure.disposition == "candidate-review" => {
            "closure-bundle-pending".to_string()
        }
        _ => row.evidence_status.clone(),
    }
}
