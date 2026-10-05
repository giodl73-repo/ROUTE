//! Helper `t2_bundle_readiness_repair_evidence_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_bundle_readiness_repair_evidence_rows(
    repair_rows: &[T2BundleReadinessRepairDocketRow],
    registry_rows: &[NationalSegmentRegistryRow],
    candidate_rows: &[TierSegmentCandidateRow],
    service_rows: &[T2ServiceSelectionRow],
) -> Vec<T2BundleReadinessRepairEvidenceRow> {
    let mut rows = repair_rows
        .iter()
        .map(|repair| {
            let route_key = canonical_route_key(&repair.route);
            let (artifact, count, summary) = match repair.readiness_class.as_str() {
                "stop-chain" => {
                    let matches = registry_rows
                        .iter()
                        .filter(|row| canonical_route_key(&row.route) == route_key)
                        .count();
                    (
                        "data/national-segment-registry.csv",
                        matches,
                        format!("{matches} registry rows match route {}", repair.route),
                    )
                }
                "stitched-member" => {
                    let matches = candidate_rows
                        .iter()
                        .filter(|row| canonical_route_key(&row.route) == route_key)
                        .count();
                    (
                        "data/tier-segment-candidates.csv",
                        matches,
                        format!(
                            "{matches} segment candidate rows match route {}",
                            repair.route
                        ),
                    )
                }
                "terminal-stop" => {
                    let matches = service_rows
                        .iter()
                        .filter(|row| canonical_route_key(&row.route) == route_key)
                        .count();
                    (
                        "data/t2-service-selection.csv",
                        matches,
                        format!(
                            "{matches} service selection rows match route {}",
                            repair.route
                        ),
                    )
                }
                _ => (
                    repair.required_artifact.as_str(),
                    0,
                    format!("unsupported readiness class {}", repair.readiness_class),
                ),
            };
            let evidence_status = if count > 0 {
                "candidate-evidence-found"
            } else {
                "source-needed"
            };
            T2BundleReadinessRepairEvidenceRow {
                evidence_id: format!(
                    "T2BUNDLEREADINESSEVIDENCE-{}",
                    stable_id_fragment(&repair.repair_id)
                ),
                repair_id: repair.repair_id.clone(),
                route: repair.route.clone(),
                segment_bundle_id: repair.segment_bundle_id.clone(),
                readiness_class: repair.readiness_class.clone(),
                evidence_artifact: artifact.to_string(),
                evidence_status: evidence_status.to_string(),
                evidence_row_count: count,
                evidence_summary: summary,
                evidence_decision: "held-for-readiness-replay".to_string(),
                qualification_effects: repair.qualification_effects.clone(),
                next_artifact: "data/t2-bundle-overlay-repair-delta.csv".to_string(),
                blocks_claims: repair.blocks_claims.clone(),
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
