//! Helper `blueprint_row_contract_failure`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn blueprint_row_contract_failure(row: &BlueprintPackageRow) -> Option<String> {
    let class = row.stakeholder_class.trim().to_ascii_lowercase();
    let evidence = row.evidence_level.trim();
    let status = row.status.trim().to_ascii_lowercase();
    let class_ok = matches!(
        class.as_str(),
        "operational_must_have"
            | "source_gated_must_have"
            | "conditional_expansion"
            | "mitigation_companion"
    );
    let status_ok = matches!(
        status.as_str(),
        "blueprint_candidate" | "held" | "backlog" | "downgraded"
    );
    let filled = !row.package_id.trim().is_empty()
        && !row.phase.trim().is_empty()
        && !row.feature_package.trim().is_empty()
        && class_ok
        && !row.standards.trim().is_empty()
        && standards_evidence_level_is_allowed(evidence)
        && status_ok
        && !row.cost_range.trim().is_empty()
        && !row.value_case.trim().is_empty()
        && !row.source_label.trim().is_empty()
        && !row.pressure_artifact.trim().is_empty()
        && !row.forum_constraint.trim().is_empty()
        && !row.mitigation_companion.trim().is_empty()
        && !row.row_complexity.trim().is_empty()
        && !row.maintenance_burden.trim().is_empty()
        && !row.community_exposure_check.trim().is_empty()
        && !row.rural_access_exception.trim().is_empty()
        && !row.blueprint_action.trim().is_empty()
        && !row.blocking_gap.trim().is_empty()
        && !row.next_evidence_step.trim().is_empty();

    if filled {
        None
    } else {
        Some(format!(
            "{} invalid contract: class={} evidence={} status={}",
            if row.package_id.trim().is_empty() {
                "<missing-package-id>"
            } else {
                row.package_id.as_str()
            },
            row.stakeholder_class,
            row.evidence_level,
            row.status
        ))
    }
}
