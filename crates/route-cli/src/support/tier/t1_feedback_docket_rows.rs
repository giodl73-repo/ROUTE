//! Helper `t1_feedback_docket_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_feedback_docket_rows(
    service_rows: &[T2ServiceSelectionRow],
    bubble_rows: &[T2BubbleUpReviewRow],
    intake_rows: &[T3T4PressureIntakeRow],
    sla_rows: &[T1SlaPairRow],
) -> Vec<T1FeedbackDocketRow> {
    let sla_pairs_by_route = t1_sla_pairs_by_route(sla_rows);
    let mut rows = Vec::new();
    let mut seen_sources = std::collections::BTreeSet::<(String, String)>::new();

    for row in service_rows {
        let route_key = canonical_route_key(&row.route);
        let pairs = sla_pairs_by_route
            .get(&route_key)
            .cloned()
            .unwrap_or_default();
        let (class, action, required_evidence, next_artifact, optimizer_effect, status) =
            t1_feedback_service_decision(row, !pairs.is_empty());
        seen_sources.insert((route_key, "t2-service-selection".to_string()));
        rows.push(T1FeedbackDocketRow {
            route: row.route.clone(),
            source_surface: "t2-service-selection".to_string(),
            source_action: row.selection_action.clone(),
            current_score: 0.0,
            t1_feedback_class: class.to_string(),
            t1_feedback_action: action.to_string(),
            t1_sla_pair_count: pairs.len(),
            t1_sla_pairs: pairs.join(";"),
            required_evidence: required_evidence.to_string(),
            next_artifact: next_artifact.to_string(),
            optimizer_effect: optimizer_effect.to_string(),
            validation_status: status.to_string(),
        });
    }

    for row in bubble_rows {
        let route_key = canonical_route_key(&row.route);
        let pairs = sla_pairs_by_route
            .get(&route_key)
            .cloned()
            .unwrap_or_default();
        let (class, action, required_evidence, next_artifact, optimizer_effect, status) =
            t1_feedback_bubble_decision(!pairs.is_empty());
        seen_sources.insert((route_key, "t2-bubble-up-review".to_string()));
        rows.push(T1FeedbackDocketRow {
            route: row.route.clone(),
            source_surface: "t2-bubble-up-review".to_string(),
            source_action: row.review_action.clone(),
            current_score: row.current_score,
            t1_feedback_class: class.to_string(),
            t1_feedback_action: action.to_string(),
            t1_sla_pair_count: pairs.len(),
            t1_sla_pairs: pairs.join(";"),
            required_evidence: required_evidence.to_string(),
            next_artifact: next_artifact.to_string(),
            optimizer_effect: optimizer_effect.to_string(),
            validation_status: status.to_string(),
        });
    }

    for row in intake_rows.iter().filter(|row| {
        row.current_score >= T1_THRESHOLD - 5.0 || row.intake_class == "bubble-up-t2-review"
    }) {
        let route_key = canonical_route_key(&row.route);
        if seen_sources.contains(&(route_key.clone(), "t2-bubble-up-review".to_string())) {
            continue;
        }
        let pairs = sla_pairs_by_route
            .get(&route_key)
            .cloned()
            .unwrap_or_default();
        let (class, action, required_evidence, next_artifact, optimizer_effect, status) =
            t1_feedback_intake_decision(row, !pairs.is_empty());
        rows.push(T1FeedbackDocketRow {
            route: row.route.clone(),
            source_surface: "t3-t4-pressure-intake".to_string(),
            source_action: row.intake_action.clone(),
            current_score: row.current_score,
            t1_feedback_class: class.to_string(),
            t1_feedback_action: action.to_string(),
            t1_sla_pair_count: pairs.len(),
            t1_sla_pairs: pairs.join(";"),
            required_evidence: required_evidence.to_string(),
            next_artifact: next_artifact.to_string(),
            optimizer_effect: optimizer_effect.to_string(),
            validation_status: status.to_string(),
        });
    }

    rows.sort_by(|a, b| {
        a.t1_feedback_class
            .cmp(&b.t1_feedback_class)
            .then_with(|| b.t1_sla_pair_count.cmp(&a.t1_sla_pair_count))
            .then_with(|| b.current_score.total_cmp(&a.current_score))
            .then_with(|| a.route.cmp(&b.route))
            .then_with(|| a.source_surface.cmp(&b.source_surface))
    });
    rows
}

