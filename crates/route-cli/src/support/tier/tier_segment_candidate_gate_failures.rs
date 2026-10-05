//! Helper `tier_segment_candidate_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_segment_candidate_gate_failures(
    rows: &[TierSegmentCandidateRow],
    t1_rows: &[T1LineSelectorInputRow],
    t2_rows: &[T2ServiceSelectionRow],
    repair_rows: &[T2BundleRepairQueueRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no tier segment candidate rows emitted".to_string());
        return failures;
    }
    let mut seen_segments = std::collections::BTreeSet::<String>::new();
    let mut rows_by_service = std::collections::BTreeMap::<(String, String), usize>::new();
    for row in rows {
        if row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.national_segment_id.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.stitch_group_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.route_aliases.trim().is_empty()
            || row.candidate_action.trim().is_empty()
        {
            failures.push(format!(
                "{}:{} has incomplete segment candidate fields",
                row.tier, row.route
            ));
        }
        if !row.national_segment_id.starts_with("US.HWYSEG.") {
            failures.push(format!("{} is not a segment id", row.national_segment_id));
        }
        if !row.segment_bundle_id.starts_with("US.HWYBUNDLE.") {
            failures.push(format!("{} is not a bundle id", row.segment_bundle_id));
        }
        if !row.stitch_group_id.starts_with("US.HWYSTITCH.") {
            failures.push(format!("{} is not a stitch id", row.stitch_group_id));
        }
        if !seen_segments.insert(format!(
            "{}|{}|{}",
            row.segment_bundle_id, row.edge_sequence, row.national_segment_id
        )) {
            failures.push(format!(
                "{} repeats segment {} at sequence {}",
                row.segment_bundle_id, row.national_segment_id, row.edge_sequence
            ));
        }
        *rows_by_service
            .entry((row.tier.clone(), row.route.clone()))
            .or_default() += 1;
        if !row.qualification_effects.trim().is_empty() && !row.source_selector.starts_with("t2-") {
            failures.push(format!(
                "{}:{} segment candidate carries untraceable qualification effects",
                row.tier, row.route
            ));
        }
    }

    for row in t1_rows.iter().filter(|row| row.selected) {
        let key = ("T1".to_string(), normalise_designation(&row.route));
        if !rows_by_service.contains_key(&key) {
            failures.push(format!(
                "selected T1 {} has no segment candidates",
                row.route
            ));
        }
    }
    for row in t2_rows.iter().filter(|row| {
        matches!(
            row.selection_action.as_str(),
            "keep-service-column" | "parent-region-review" | "source-needed"
        )
    }) {
        let key = ("T2".to_string(), normalise_designation(&row.route));
        if !rows_by_service.contains_key(&key) {
            failures.push(format!(
                "T2 service {} has no segment candidates",
                row.route
            ));
        }
    }
    for row in repair_rows
        .iter()
        .filter(|row| row.next_artifact == "data/national-segment-bundles.csv")
    {
        let key = ("T2".to_string(), normalise_designation(&row.route));
        if !rows_by_service.contains_key(&key) {
            failures.push(format!(
                "T2 bundle repair {} has no segment candidates",
                row.route
            ));
        }
    }

    failures
}
