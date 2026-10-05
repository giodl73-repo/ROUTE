//! Helper `national_segment_bundle_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn national_segment_bundle_gate_failures(
    rows: &[NationalSegmentBundleRow],
    registry_rows: &[NationalSegmentRegistryRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no national segment bundle rows emitted".to_string());
        return failures;
    }

    let registry_segment_ids = registry_rows
        .iter()
        .map(|row| row.national_segment_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let registry_bundle_ids = registry_rows
        .iter()
        .map(|row| row.segment_bundle_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let mut expected_effects_by_bundle =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    for row in registry_rows {
        insert_pipe_values(
            expected_effects_by_bundle
                .entry(row.segment_bundle_id.clone())
                .or_default(),
            &row.qualification_effects,
        );
    }

    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.segment_bundle_id.trim().is_empty()
            || row.bundle_role.trim().is_empty()
            || row.member_segment_ids.trim().is_empty()
            || row.member_count == 0
            || row.stitch_group_ids.trim().is_empty()
            || row.current_tiers.trim().is_empty()
            || row.current_zone_ids.trim().is_empty()
            || row.bundle_aliases.trim().is_empty()
            || row.source_artifacts.trim().is_empty()
            || row.bundle_status.trim().is_empty()
            || row.bundle_action.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete bundle fields",
                row.segment_bundle_id
            ));
        }
        if !seen.insert(row.segment_bundle_id.clone()) {
            failures.push(format!("{} is duplicated", row.segment_bundle_id));
        }
        if !row.segment_bundle_id.starts_with("US.HWYBUNDLE.") {
            failures.push(format!("{} is not a bundle id", row.segment_bundle_id));
        }
        if !registry_bundle_ids.contains(&row.segment_bundle_id) {
            failures.push(format!(
                "{} has no backing segment-registry row",
                row.segment_bundle_id
            ));
        }
        let member_ids = semicolon_values(&row.member_segment_ids);
        if member_ids.len() != row.member_count {
            failures.push(format!(
                "{} member_count {} does not match member list {}",
                row.segment_bundle_id,
                row.member_count,
                member_ids.len()
            ));
        }
        for member_id in &member_ids {
            if !registry_segment_ids.contains(member_id) {
                failures.push(format!(
                    "{} references unknown member {}",
                    row.segment_bundle_id, member_id
                ));
            }
        }
        if row.bundle_role == "single-segment" && row.member_count != 1 {
            failures.push(format!(
                "{} single-segment bundle has {} members",
                row.segment_bundle_id, row.member_count
            ));
        }
        if row.bundle_role != "single-segment"
            && row.member_count < 2
            && row.validation_status == "pass"
        {
            failures.push(format!(
                "{} non-single bundle passed with fewer than two members",
                row.segment_bundle_id
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{} has invalid validation status {}",
                row.segment_bundle_id, row.validation_status
            ));
        }
        if let Some(expected_effects) = expected_effects_by_bundle.get(&row.segment_bundle_id) {
            let expected_effects = join_pipe_set(expected_effects);
            if !expected_effects.is_empty() && row.qualification_effects != expected_effects {
                failures.push(format!(
                    "{} drops qualification effects from registry members",
                    row.segment_bundle_id
                ));
            }
        }
    }

    failures
}
