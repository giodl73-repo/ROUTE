//! Helper `blueprint_cost_row_failure`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn blueprint_cost_row_failure(
    row: &BlueprintCostRow,
    package_ids: &std::collections::HashSet<&str>,
) -> Option<String> {
    let source_status = row.source_status.trim().to_ascii_lowercase();
    let claim_status = row.cost_claim_status.trim().to_ascii_lowercase();
    let source_status_ok = matches!(
        source_status.as_str(),
        "source_backed" | "planning_range" | "corridor_specific" | "source_needed"
    );
    let claim_status_ok = matches!(
        claim_status.as_str(),
        "source_backed" | "planning_only" | "placeholder" | "held"
    );
    let no_premature_source_claim =
        source_status == "source_backed" || claim_status != "source_backed";
    let filled = !row.package_id.trim().is_empty()
        && package_ids.contains(row.package_id.trim())
        && !row.cost_basis.trim().is_empty()
        && !row.capital_range_2026_usd.trim().is_empty()
        && !row.lifecycle_burden.trim().is_empty()
        && source_status_ok
        && !row.source_artifact.trim().is_empty()
        && claim_status_ok
        && no_premature_source_claim
        && !row.risk_note.trim().is_empty()
        && !row.next_cost_step.trim().is_empty();

    if filled {
        None
    } else {
        Some(format!(
            "{} invalid cost row: source={} claim={}",
            if row.package_id.trim().is_empty() {
                "<missing-package-id>"
            } else {
                row.package_id.as_str()
            },
            row.source_status,
            row.cost_claim_status
        ))
    }
}

