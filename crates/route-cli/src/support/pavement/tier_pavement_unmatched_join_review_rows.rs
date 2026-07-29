//! Helper `tier_pavement_unmatched_join_review_rows` (support::pavement).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_unmatched_join_review_rows(
    fetch_review_rows: &[TierPavementSourceFetchReviewRow],
    source_gap_rows: &[TierPavementSourceGapRow],
    docket_rows: &[TierPavementDocketRow],
    cache_dir: &Path,
) -> Result<Vec<TierPavementUnmatchedJoinReviewRow>> {
    let source_gap_bundles_by_state = source_gap_rows
        .iter()
        .flat_map(|row| {
            row.affected_states
                .split(';')
                .map(str::trim)
                .filter(|state| !state.is_empty())
                .map(move |state| (state.to_string(), row.segment_bundle_id.clone()))
        })
        .fold(
            std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new(),
            |mut acc, (state, bundle)| {
                acc.entry(state).or_default().insert(bundle);
                acc
            },
        );
    let mut rows = Vec::new();
    for fetch_review in fetch_review_rows {
        let cache_path = cache_dir.join(format!("hpms_{}.csv", fetch_review.state.to_lowercase()));
        let hpms_records = route_data::hpms::read_hpms_csv(&cache_path)
            .with_context(|| format!("loading {}", cache_path.display()))?;
        let source_gap_bundles = source_gap_bundles_by_state
            .get(&fetch_review.state)
            .cloned()
            .unwrap_or_default();
        let state_gap_rows = docket_rows
            .iter()
            .filter(|row| {
                row.state == fetch_review.state
                    && source_gap_bundles.contains(row.segment_bundle_id.as_str())
                    && row.validation_status == "review"
            })
            .collect::<Vec<_>>();
        let source_needed_rows = state_gap_rows
            .iter()
            .copied()
            .filter(|row| row.pavement_status == "pavement-source-needed")
            .collect::<Vec<_>>();
        let repair_required_rows = state_gap_rows
            .iter()
            .copied()
            .filter(|row| row.pavement_status == "pavement-repair-required")
            .collect::<Vec<_>>();
        let source_needed_routes = source_needed_rows
            .iter()
            .map(|row| row.route.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let repair_required_routes = repair_required_rows
            .iter()
            .map(|row| row.route.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let hpms_records_for_source_needed_routes = hpms_records
            .iter()
            .filter(|record| {
                record.state == fetch_review.state
                    && source_needed_routes.contains(record.route_id.as_str())
                    && record.iri.is_some()
            })
            .count();
        let hpms_source_route_coverage = if source_needed_routes.is_empty() {
            "not-needed"
        } else if hpms_records_for_source_needed_routes == 0 {
            "none"
        } else {
            let covered_routes = hpms_records
                .iter()
                .filter(|record| {
                    record.state == fetch_review.state
                        && source_needed_routes.contains(record.route_id.as_str())
                        && record.iri.is_some()
                })
                .map(|record| record.route_id.clone())
                .collect::<std::collections::BTreeSet<_>>();
            if covered_routes.len() == source_needed_routes.len() {
                "complete-route-records"
            } else {
                "partial-route-records"
            }
        };
        let join_review_status =
            if !source_needed_routes.is_empty() && hpms_records_for_source_needed_routes == 0 {
                "hpms-scope-misses-source-needed-routes"
            } else if !source_needed_routes.is_empty() {
                "hpms-route-records-present-join-still-open"
            } else if !repair_required_routes.is_empty() {
                "repair-debt-not-source-join"
            } else {
                "no-open-priority-a-pavement-gap"
            };
        let next_action = match join_review_status {
            "hpms-scope-misses-source-needed-routes" => {
                "attach state DOT pavement condition evidence or broaden HPMS fetch scope for source-needed routes before relief"
            }
            "hpms-route-records-present-join-still-open" => {
                "review route-to-member join keys before accepting HPMS pavement evidence"
            }
            "repair-debt-not-source-join" => {
                "route repair debt to pavement repair funding review, not source acquisition"
            }
            _ => "no priority-A pavement join action required",
        };
        rows.push(TierPavementUnmatchedJoinReviewRow {
            join_review_id: format!(
                "PAVEMENTJOINREVIEW-{}",
                stable_id_fragment(&fetch_review.state)
            ),
            state: fetch_review.state.clone(),
            source_priority: fetch_review.source_priority.clone(),
            cache_record_count: fetch_review.cache_record_count,
            source_gap_member_count: state_gap_rows.len(),
            source_needed_member_count: source_needed_rows.len(),
            repair_required_member_count: repair_required_rows.len(),
            source_needed_routes: join_string_set(&source_needed_routes),
            repair_required_routes: join_string_set(&repair_required_routes),
            hpms_records_for_source_needed_routes,
            hpms_source_route_coverage: hpms_source_route_coverage.to_string(),
            join_review_status: join_review_status.to_string(),
            evidence_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: fetch_review.blocker_claims_before.clone(),
            blocker_claims_after: fetch_review.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_action: next_action.to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        });
    }
    Ok(rows)
}

