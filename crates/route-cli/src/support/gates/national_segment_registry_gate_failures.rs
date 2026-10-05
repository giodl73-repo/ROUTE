//! Helper `national_segment_registry_gate_failures` (support::gates).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn national_segment_registry_gate_failures(
    rows: &[NationalSegmentRegistryRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no national segment registry rows emitted".to_string());
        return failures;
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.national_segment_id.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.stitch_group_id.trim().is_empty()
            || row.current_zone_id.trim().is_empty()
            || row.current_tier.trim().is_empty()
            || row.segment_aliases.trim().is_empty()
            || row.bundle_aliases.trim().is_empty()
            || row.board_layers.trim().is_empty()
            || row.source_artifacts.trim().is_empty()
            || row.bundle_role.trim().is_empty()
            || row.member_segment_ids.trim().is_empty()
            || row.registry_action.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete registry fields",
                row.national_segment_id
            ));
        }
        if !seen.insert(national_segment_member_key(
            &row.segment_bundle_id,
            &row.national_segment_id,
        )) {
            failures.push(format!(
                "{} is duplicated in bundle {}",
                row.national_segment_id, row.segment_bundle_id
            ));
        }
        if !row.national_segment_id.starts_with("US.HWYSEG.") {
            failures.push(format!(
                "{} is not a highway segment id",
                row.national_segment_id
            ));
        }
        if !row.segment_bundle_id.starts_with("US.HWYBUNDLE.") {
            failures.push(format!(
                "{} has invalid bundle id {}",
                row.national_segment_id, row.segment_bundle_id
            ));
        }
        if !row.stitch_group_id.starts_with("US.HWYSTITCH.") {
            failures.push(format!(
                "{} has invalid stitch group {}",
                row.national_segment_id, row.stitch_group_id
            ));
        }
        if row.national_segment_id.contains("T1")
            || row.national_segment_id.contains("T2")
            || row.national_segment_id.contains("T3")
            || row.national_segment_id.contains("T4")
            || row.national_segment_id.contains("GREAT")
            || row.national_segment_id.contains("SOUTHEAST")
            || row.national_segment_id.contains("MOUNTAIN")
            || row.national_segment_id.contains("TEXAS")
        {
            failures.push(format!(
                "{} leaks tier or zone semantics into stable segment id",
                row.national_segment_id
            ));
        }
        if !matches!(row.current_tier.as_str(), "T1" | "T2" | "T3" | "T4") {
            failures.push(format!(
                "{} has invalid current tier {}",
                row.national_segment_id, row.current_tier
            ));
        }
        if row.bundle_role == "single-segment" && row.member_segment_ids != row.national_segment_id
        {
            failures.push(format!(
                "{} single-segment bundle has mismatched members {}",
                row.national_segment_id, row.member_segment_ids
            ));
        }
        if !row.qualification_effects.trim().is_empty()
            && (!row.board_layers.contains("tier-segment-candidate")
                || !row
                    .source_artifacts
                    .contains("data/tier-segment-candidates.csv"))
        {
            failures.push(format!(
                "{} carries qualification effects without segment candidate source",
                row.national_segment_id
            ));
        }
        if row.board_layers.contains("selected-route")
            && row.stop_placement_status.trim().is_empty()
        {
            failures.push(format!(
                "{} selected route missing stop placement status",
                row.national_segment_id
            ));
        }
        if matches!(row.current_tier.as_str(), "T1" | "T2")
            && !row.stop_placement_status.contains("pavement-")
        {
            failures.push(format!(
                "{} {} member lacks pavement readiness status",
                row.current_tier, row.national_segment_id
            ));
        }
        if row.stop_placement_status.contains("ready-for-stop-layout")
            && row.evidence_state_scope.trim().is_empty()
            && row.geometry_state_scope.trim().is_empty()
        {
            failures.push(format!(
                "{} ready route missing state scope",
                row.national_segment_id
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.national_segment_id, row.validation_status
            ));
        }
    }

    failures
}
