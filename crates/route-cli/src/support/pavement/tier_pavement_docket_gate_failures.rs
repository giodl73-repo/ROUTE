//! Helper `tier_pavement_docket_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_docket_gate_failures(
    rows: &[TierPavementDocketRow],
    segment_rows: &[TierSegmentCandidateRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no tier pavement docket rows emitted".to_string());
        return failures;
    }
    if rows.len() != segment_rows.len() {
        failures.push(format!(
            "pavement docket row count {} does not match segment candidate count {}",
            rows.len(),
            segment_rows.len()
        ));
    }

    let mut seen = std::collections::BTreeSet::<(String, u64)>::new();
    let segment_effects_by_member = segment_rows
        .iter()
        .filter(|row| !row.qualification_effects.trim().is_empty())
        .map(|row| {
            (
                (
                    row.segment_bundle_id.clone(),
                    row.national_segment_id.clone(),
                    row.edge_id,
                ),
                row.qualification_effects.clone(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    for row in rows {
        if row.tier.trim().is_empty()
            || row.route.trim().is_empty()
            || row.segment_bundle_id.trim().is_empty()
            || row.stitch_group_id.trim().is_empty()
            || row.national_segment_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.iri_m_per_km.trim().is_empty()
            || row.max_iri_m_per_km.trim().is_empty()
            || row.pavement_status.trim().is_empty()
            || row.repair_action.trim().is_empty()
            || row.source_contract.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{}:{} edge {} has incomplete pavement docket fields",
                row.tier, row.route, row.edge_id
            ));
        }
        if !row.segment_bundle_id.starts_with("US.HWYBUNDLE.") {
            failures.push(format!("{} is not a bundle id", row.segment_bundle_id));
        }
        if !row.national_segment_id.starts_with("US.HWYSEG.") {
            failures.push(format!("{} is not a segment id", row.national_segment_id));
        }
        if !seen.insert((row.segment_bundle_id.clone(), row.edge_id)) {
            failures.push(format!(
                "{} repeats pavement edge {}",
                row.segment_bundle_id, row.edge_id
            ));
        }
        if !matches!(
            row.pavement_status.as_str(),
            "pavement-floor-pass"
                | "pavement-repair-required"
                | "pavement-source-needed"
                | "missing-tier-standard"
                | "missing-graph-edge"
        ) {
            failures.push(format!(
                "{}:{} edge {} has unknown pavement status {}",
                row.tier, row.route, row.edge_id, row.pavement_status
            ));
        }
        if !matches!(row.validation_status.as_str(), "pass" | "review") {
            failures.push(format!(
                "{}:{} edge {} has invalid validation status {}",
                row.tier, row.route, row.edge_id, row.validation_status
            ));
        }
        if row.validation_status == "pass" && row.pavement_status != "pavement-floor-pass" {
            failures.push(format!(
                "{}:{} edge {} passes without a pavement-floor-pass status",
                row.tier, row.route, row.edge_id
            ));
        }
        if let Some(expected_effects) = segment_effects_by_member.get(&(
            row.segment_bundle_id.clone(),
            row.national_segment_id.clone(),
            row.edge_id,
        )) {
            if row.qualification_effects != *expected_effects {
                failures.push(format!(
                    "{}:{} edge {} drops qualification effects",
                    row.tier, row.route, row.edge_id
                ));
            }
        }
    }
    failures
}
