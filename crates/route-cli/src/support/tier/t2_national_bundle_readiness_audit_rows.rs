//! Helper `t2_national_bundle_readiness_audit_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_national_bundle_readiness_audit_rows(
    replay_rows: &[T2BundleReadinessReplayDecisionRow],
    bundle_rows: &[NationalSegmentBundleRow],
) -> Vec<T2NationalBundleReadinessAuditRow> {
    let bundles_by_id = bundle_rows
        .iter()
        .map(|row| (row.segment_bundle_id.as_str(), row))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = replay_rows
        .iter()
        .filter(|row| row.next_artifact == "data/national-segment-bundles.csv")
        .map(|replay| {
            let bundle = bundles_by_id.get(replay.segment_bundle_id.as_str());
            let (bundle_status, bundle_validation_status, bundle_member_count, next_artifact) =
                bundle
                    .map(|row| {
                        (
                            row.bundle_status.clone(),
                            row.validation_status.clone(),
                            row.member_count,
                            row.next_artifact.clone(),
                        )
                    })
                    .unwrap_or_else(|| {
                        (
                            "missing-bundle-row".to_string(),
                            "review".to_string(),
                            0,
                            "data/national-segment-registry.csv".to_string(),
                        )
                    });
            let audit_action = match bundle_status.as_str() {
                "needs-stop-chain" => "author-stop-chain-before-replay",
                "needs-stitched-members" => "stitch-member-segments-before-replay",
                "needs-terminal-stop" => "author-terminal-stop-before-replay",
                "bundle-ready" => "manual-review-before-claim-promotion",
                "missing-bundle-row" => "restore-bundle-row-before-replay",
                _ => "manual-bundle-readiness-review",
            };
            T2NationalBundleReadinessAuditRow {
                audit_id: format!(
                    "T2NATIONALBUNDLEAUDIT-{}",
                    stable_id_fragment(&replay.replay_id)
                ),
                replay_id: replay.replay_id.clone(),
                route: replay.route.clone(),
                segment_bundle_id: replay.segment_bundle_id.clone(),
                readiness_class: replay.readiness_class.clone(),
                replay_decision: replay.replay_decision.clone(),
                bundle_status,
                bundle_validation_status,
                bundle_member_count,
                audit_decision: "held-for-structural-bundle-repair".to_string(),
                audit_action: audit_action.to_string(),
                qualification_effects: replay.qualification_effects.clone(),
                blocked_claims_before: replay.blocked_claims_after.clone(),
                blocked_claims_after: replay.blocked_claims_after.clone(),
                blocker_delta: 0,
                next_artifact,
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.readiness_class
            .cmp(&right.readiness_class)
            .then(left.route.cmp(&right.route))
    });
    rows
}

